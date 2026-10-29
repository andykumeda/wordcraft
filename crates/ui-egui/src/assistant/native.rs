use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, mpsc};
use std::time::Duration;

use craft_ai_core::assistant::{AssistantRole, Conversation, Message, MessageRole, RuntimeConfig, Target, tool_model};
use craft_ai_core::{
    AiError, AppId, Auth, Binding, Connection, ConnectionDraft, ConnectionId, KeyChange, ModelId, ModelInfo, Prepared, SecretRef, SecretString,
    StoreFile,
};
use craft_ai_native::codex::{CodexOptions, CodexProvider};
use craft_ai_native::secrets::{KeyringStore, SecretStore};
use craft_ai_native::storage::{HistoryStore, SharedStore};
use craft_ai_native::{Cancellation, ChatProvider, HttpProvider, ModelRequest, TurnEvent, TurnRequest, resolve_choices, run_turn};
use craft_ui::assistant::{Action, Activity, ChatSummary};
use serde_json::Value;

use super::host::{self, Guard, McpHost, Pending};
use crate::WordApp;

#[derive(Default)]
pub(super) struct Native {
    initialized: bool,
    root: Option<PathBuf>,
    conversation: Option<Conversation>,
    job: Option<mpsc::Receiver<Result<Update, String>>>,
    job_cancel: Cancellation,
    turn: Option<Turn>,
    chooser: Option<mpsc::Receiver<crate::FileDialogAnswer>>,
}

struct Turn {
    events: mpsc::Receiver<WorkerEvent>,
    calls: mpsc::Receiver<Pending>,
    cancel: Cancellation,
    guard: Guard,
    fresh_text: bool,
    pending_activity: Vec<(String, usize)>,
}

impl Native {
    pub(super) fn cancel(&self) {
        self.job_cancel.cancel();
        if let Some(turn) = &self.turn {
            turn.cancel.cancel();
        }
    }
}

/// Screenshot fixtures simulate native availability without opening the user's settings or chats.
#[cfg(test)]
pub(super) fn enable_without_profile(app: &mut WordApp) {
    app.assistant.native.initialized = true;
    app.assistant.enabled = true;
}

enum WorkerEvent {
    Runtime(TurnEvent),
    Saved(Conversation),
}

enum Update {
    Loaded { root: PathBuf, store: StoreFile, history: Vec<ChatSummary> },
    Store(StoreFile),
    ConnectionSaved { change: ConnectionChange, id: ConnectionId },
    ConnectionDeleted { change: ConnectionChange, id: ConnectionId },
    Models { draft: ConnectionDraft, models: Vec<ModelInfo> },
    Chat(Conversation),
    Deleted(String),
    Login { url: String, code: String },
    Notice(String),
}

fn app_id() -> Result<AppId, String> {
    AppId::new("wordcraft").map_err(|e| e.to_string())
}
fn fail(error: impl std::fmt::Display) -> String {
    error.to_string()
}

fn history(store: &SharedStore, app: &AppId) -> Result<HistoryStore, String> {
    HistoryStore::new(store.root(), app).map_err(fail)
}
fn summaries(store: &HistoryStore) -> Result<Vec<ChatSummary>, String> {
    Ok(store.list().map_err(fail)?.into_iter().map(|c| ChatSummary { id: c.id, title: c.title, interrupted: c.interrupted }).collect())
}

fn job(app: &mut WordApp, work: impl FnOnce(mpsc::Sender<Result<Update, String>>, Cancellation) + Send + 'static) -> Result<(), String> {
    if app.assistant.native.job.is_some() {
        return Err("wait for the current assistant settings operation".into());
    }
    let (sender, receiver) = mpsc::channel();
    let cancel = Cancellation::new();
    let worker_cancel = cancel.clone();
    let wake = app.ctx.clone();
    std::thread::Builder::new()
        .name("assistant-settings".into())
        .spawn(move || {
            work(sender, worker_cancel);
            if let Some(wake) = wake {
                wake.request_repaint();
            }
        })
        .map_err(fail)?;
    app.assistant.native.job = Some(receiver);
    app.assistant.native.job_cancel = cancel;
    app.assistant.widget.settings.busy = true;
    app.assistant.widget.settings.error = None;
    app.assistant.widget.error = None;
    Ok(())
}

pub(super) fn ensure_loaded(app: &mut WordApp) {
    app.assistant.widget.available = app.assistant.enabled;
    if !app.assistant.enabled || app.assistant.native.initialized {
        return;
    }
    app.assistant.native.initialized = true;
    if let Err(error) = job(app, |sender, _| {
        let result = (|| {
            let root = SharedStore::default_root().map_err(fail)?;
            let store = SharedStore::new(root.clone());
            let history = summaries(&history(&store, &app_id()?)?)?;
            Ok(Update::Loaded { root, store: store.load().map_err(fail)?, history })
        })();
        let _ = sender.send(result);
    }) {
        app.assistant.widget.error = Some(error);
    }
}

struct Providers {
    http: HttpProvider,
    codex: CodexProvider,
}
impl Providers {
    fn new(root: &std::path::Path, secrets: Arc<dyn SecretStore>) -> Self {
        Self { http: HttpProvider::new(secrets), codex: CodexProvider::new(CodexOptions::in_shared_root(root)) }
    }
}
impl ChatProvider for Providers {
    fn complete(&self, request: &ModelRequest<'_>, cancel: &Cancellation, emit: &mut dyn FnMut(&str)) -> Result<Message, AiError> {
        if request.connection.provider == "codex" { self.codex.complete(request, cancel, emit) } else { self.http.complete(request, cancel, emit) }
    }
    fn models(&self, connection: &Connection, config: &RuntimeConfig, cancel: &Cancellation) -> Result<Vec<ModelInfo>, AiError> {
        if connection.provider == "codex" { self.codex.models(connection, config, cancel) } else { self.http.models(connection, config, cancel) }
    }
}

pub(super) fn action(app: &mut WordApp, requested: Action) -> Result<(), String> {
    ensure_loaded(app);
    if matches!(requested, Action::Stop) {
        app.assistant.native.cancel();
        app.assistant.widget.status = Some(tl!("Stopping; completed edits remain undoable.").into());
        return Ok(());
    }
    if !app.assistant.enabled {
        return Err("The assistant is available in the desktop app.".into());
    }
    if app.assistant.widget.busy {
        return Err("stop or finish the current assistant turn first".into());
    }
    if matches!(requested, Action::ReloadSettings) {
        if app.assistant.native.job.is_some() {
            return Err("wait for the current assistant settings operation".into());
        }
        app.assistant.native.initialized = false;
        ensure_loaded(app);
        return Ok(());
    }
    if let Action::AddFile(path) = requested {
        return add_file(app, path);
    }
    if let Action::RemoveFile(path) = requested {
        app.assistant.widget.authorized_files.retain(|p| *p != path);
        return Ok(());
    }
    if let Action::ChooseFiles = requested {
        if app.assistant.native.chooser.is_some() || app.file_dialog_open() {
            return Err("finish the open file chooser first".into());
        }
        let chooser = app.services.file_dialog.as_ref().ok_or("type an exact file path in the assistant to grant access")?;
        app.assistant.native.chooser = Some(chooser(crate::FileDialogRequest::OpenMany { purpose: "document".into() }));
        return Ok(());
    }
    if let Action::Send(prompt) = requested {
        let result = app
            .assistant
            .native
            .root
            .clone()
            .ok_or_else(|| "assistant settings are loading; try again when ready".to_string())
            .and_then(|root| send(app, root, prompt.clone()));
        if result.is_err() && app.assistant.widget.draft.is_empty() {
            app.assistant.widget.draft = prompt;
        }
        return result;
    }
    let root = app.assistant.native.root.clone().ok_or("assistant settings are loading; try again when ready")?;
    let app_id = app_id()?;
    job(app, move |sender, cancel| {
        let store = SharedStore::new(root);
        if matches!(requested, Action::BeginCodexSignIn) {
            let result = login(&store, &sender, &cancel);
            let _ = sender.send(result);
        } else {
            let _ = sender.send(settings_action(&store, &app_id, requested, &cancel));
        }
    })
}

fn add_file(app: &mut WordApp, path: String) -> Result<(), String> {
    let path = path.trim().to_string();
    if !std::path::Path::new(&path).is_absolute() {
        return Err("use an absolute path for an authorized file".into());
    }
    let mut files = app.assistant.widget.authorized_files.clone();
    if !files.contains(&path) {
        files.push(path);
    }
    wordcraft_engine::file_access::FileGrants::new(&files)?;
    app.assistant.widget.authorized_files = files;
    Ok(())
}

fn send(app: &mut WordApp, root: PathBuf, prompt: String) -> Result<(), String> {
    if app.assistant.native.job.is_some() {
        return Err("wait for assistant settings to finish loading".into());
    }
    let app_id = app_id()?;
    let wake = app.ctx.clone().ok_or("the assistant needs an active app window")?;
    let target = Target { id: app.session.document_id().to_string(), revision: app.session.rev() };
    let mut conversation = app.assistant.native.conversation.clone();
    let mut messages = conversation.as_ref().map(|c| c.messages.clone()).unwrap_or_default();
    messages.push(Message::text(MessageRole::User, prompt.clone()));
    let (events_tx, events) = mpsc::channel();
    let (call_tx, calls) = mpsc::channel();
    let cancel = Cancellation::new();
    let worker_cancel = cancel.clone();
    let thread_messages = messages.clone();
    let title: String = prompt.chars().take(80).collect();
    let guard = Guard::new(app, prompt, app.assistant.widget.authorized_files.clone());
    std::thread::Builder::new()
        .name("craft-assistant".into())
        .spawn(move || {
            let result = (|| -> Result<(), String> {
                let shared = SharedStore::new(root.clone());
                let history = history(&shared, &app_id)?;
                if conversation.is_none() {
                    conversation = Some(history.create(&title).map_err(fail)?);
                }
                let Some(mut saved) = conversation else {
                    return Err("could not create the conversation".into());
                };
                saved.messages = thread_messages.clone();
                saved.interrupted = true;
                history.save(&saved).map_err(fail)?;
                let _ = events_tx.send(WorkerEvent::Saved(saved.clone()));
                let config = shared.config().map_err(fail)?;
                let (chat, control) = resolve_choices(&shared.load().map_err(fail)?, &app_id).map_err(fail)?;
                let provider = Providers::new(&root, Arc::new(KeyringStore));
                let mut host = McpHost::new(call_tx, worker_cancel.clone(), wake.clone(), Duration::from_secs(config.request_timeout_secs));
                let request = TurnRequest { app: app_id, messages: thread_messages, target, chat, control, config };
                let mut partial = String::new();
                let mut persistence_error = None;
                let outcome = run_turn(request, &provider, &mut host, &worker_cancel, |event| {
                    match &event {
                        TurnEvent::Routed { .. } => partial.clear(),
                        TurnEvent::Text { text, .. } => partial.push_str(text),
                        TurnEvent::Checkpoint { messages } => {
                            partial.clear();
                            if let Err(error) = save_checkpoint(&history, &mut saved, messages, &worker_cancel) {
                                persistence_error.get_or_insert(error);
                            }
                            // Checkpoints are durable recovery state; streamed text remains on screen.
                            return;
                        }
                        _ => {}
                    }
                    let _ = events_tx.send(WorkerEvent::Runtime(event));
                    wake.request_repaint();
                });
                match outcome {
                    Ok(messages) => {
                        saved.messages = messages;
                        saved.interrupted = false;
                    }
                    Err(error) => {
                        if !partial.is_empty() {
                            saved.messages.push(Message::text(MessageRole::Assistant, partial));
                        }
                        saved.messages.push(Message::text(
                            MessageRole::Assistant,
                            format!("Interrupted: {error}. Completed edits may remain; inspect the document before requesting more changes."),
                        ));
                    }
                }
                if let Err(error) = history.save(&saved) {
                    saved.interrupted = true;
                    persistence_error.get_or_insert_with(|| error.to_string());
                }
                // Deliver the latest transcript even when its final disk write failed. Continuing
                // manually must retain known command outcomes and must never replay them.
                let _ = events_tx.send(WorkerEvent::Saved(saved));
                if let Some(error) = persistence_error {
                    return Err(format!("Conversation could not be saved: {error}. The latest transcript is still visible in this window."));
                }
                Ok(())
            })();
            if let Err(message) = result {
                let _ = events_tx.send(WorkerEvent::Runtime(TurnEvent::Failed { message }));
            }
            wake.request_repaint();
        })
        .map_err(fail)?;
    app.assistant.widget.messages = messages;
    app.assistant.widget.activity.clear();
    app.assistant.widget.error = None;
    app.assistant.widget.busy = true;
    app.assistant.widget.status = Some(tl!("Connecting…").into());
    app.assistant.native.turn = Some(Turn { events, calls, cancel, guard, fresh_text: true, pending_activity: Vec::new() });
    Ok(())
}

fn save_checkpoint(history: &HistoryStore, saved: &mut Conversation, messages: &[Message], cancel: &Cancellation) -> Result<(), String> {
    saved.messages = messages.to_vec();
    saved.interrupted = true;
    history.save(saved).map_err(|error| {
        // Runtime checks cancellation before dispatching the next host request.
        cancel.cancel();
        error.to_string()
    })
}

fn settings_action(store: &SharedStore, app: &AppId, requested: Action, cancel: &Cancellation) -> Result<Update, String> {
    cancel.check().map_err(fail)?;
    match requested {
        Action::SaveConnection(prepared) => {
            let id = prepared.connection.id.clone();
            save_connection(store, prepared, &KeyringStore).map(|change| Update::ConnectionSaved { change, id })
        }
        Action::DiscoverModels(draft) | Action::TestConnection(draft) => discover(store, draft, None, cancel),
        Action::VerifyModel { connection, model } => discover(store, connection, Some(model), cancel),
        Action::SaveBinding { role, connection, model, app_override, manual_tools } => {
            let connection = ConnectionId::new(connection).map_err(fail)?;
            let model = ModelId::new(model).map_err(fail)?;
            let declaration = role.declaration().map_err(fail)?;
            let stored = store
                .update(|stored| {
                    let selected =
                        stored.connections.iter_mut().find(|c| c.id == connection).ok_or_else(|| AiError::invalid("select a saved connection"))?;
                    if manual_tools {
                        let model_info = tool_model(model.clone())?;
                        selected.models.retain(|m| m.id != model);
                        selected.models.push(model_info);
                    }
                    let binding = Binding { role: declaration.id.clone(), connection: connection.clone(), model: model.clone() };
                    let resolved = craft_ai_core::role::resolve(&declaration, &[std::slice::from_ref(&binding)], &stored.connections)
                        .map_err(|e| AiError::invalid(e.to_string()))?;
                    if !resolved.spec.features.tools {
                        return Err(AiError::unsupported("verify tool support before selecting this model"));
                    }
                    let bindings = if app_override { &mut stored.apps.entry(app.clone()).or_default().overrides } else { &mut stored.defaults };
                    bindings.retain(|b| b.role != binding.role);
                    bindings.push(binding);
                    Ok(())
                })
                .map_err(fail)?;
            Ok(Update::Store(stored))
        }
        Action::UseSuiteDefault { role } => Ok(Update::Store(
            store
                .update(|stored| {
                    if let Some(settings) = stored.apps.get_mut(app) {
                        settings.overrides.retain(|binding| binding.role.as_str() != role.id());
                    }
                    Ok(())
                })
                .map_err(fail)?,
        )),
        Action::DeleteConnection(id) => delete_connection(store, &id, &KeyringStore).map(|change| Update::ConnectionDeleted { change, id }),
        Action::NewChat => Ok(Update::Chat(history(store, app)?.create("New chat").map_err(fail)?)),
        Action::OpenChat(id) => Ok(Update::Chat(history(store, app)?.load(&id).map_err(fail)?)),
        Action::DeleteChat(id) => {
            history(store, app)?.delete(&id).map_err(fail)?;
            Ok(Update::Deleted(id))
        }
        Action::RefreshCodex => refresh_codex(store, cancel),
        _ => Err("this assistant action is not a settings operation".into()),
    }
}

struct ConnectionChange {
    store: StoreFile,
    warning: Option<String>,
}

static NEXT_SECRET: AtomicU64 = AtomicU64::new(1);

// Stage a replacement under a fresh locator. A failed settings write must never destroy the
// credential used by the previously saved connection, even if another Craft app is running.
fn save_connection(store: &SharedStore, mut prepared: Prepared, secrets: &dyn SecretStore) -> Result<ConnectionChange, String> {
    prepared.connection.validate().map_err(fail)?;
    let staged = if let KeyChange::Set(value) = &prepared.key {
        let Auth::ApiKey { secret } = &mut prepared.connection.auth else {
            return Err("Only API connections accept a stored API key.".into());
        };
        *secret = SecretRef::new(format!(
            "craft-key-{}-{}-{}",
            chrono::Utc::now().timestamp_micros(),
            std::process::id(),
            NEXT_SECRET.fetch_add(1, Ordering::Relaxed)
        ))
        .map_err(fail)?;
        secrets.set(secret, value).map_err(|_| {
            "The system credential store could not save the key. The previous connection is unchanged; keep the key in this form and try again."
                .to_string()
        })?;
        Some(secret.clone())
    } else {
        None
    };
    let id = prepared.connection.id.clone();
    let mut previous = None;
    let saved = store.update(|stored| {
        if let Some(current) = stored.connections.iter().find(|connection| connection.id == id) {
            if let Auth::ApiKey { secret } = &current.auth {
                previous = Some(secret.clone());
            }
            if matches!(prepared.key, KeyChange::Keep) && matches!(prepared.connection.auth, Auth::ApiKey { .. }) {
                // An empty key field means keep the latest acknowledged key, not a stale locator.
                prepared.connection.auth = current.auth.clone();
            }
        }
        stored.connections.retain(|connection| connection.id != id);
        stored.connections.push(prepared.connection);
        Ok(())
    });
    match saved {
        Ok(saved) => Ok(ConnectionChange { warning: cleanup_key(store, secrets, previous.as_ref()), store: saved }),
        Err(error) => {
            // A filesystem error may occur after atomic rename. Never delete the replacement
            // credential unless a fresh read proves that no committed connection references it.
            let suffix = if let Some(reference) = staged.as_ref() {
                match store.load() {
                    Ok(current) if current.connections.iter().any(|connection| connection.auth.secret_ref() == Some(reference)) => {
                        return Ok(ConnectionChange {
                            store: current,
                            warning: Some(
                                "The connection was saved, but the settings write reported a filesystem error. The replacement key was retained."
                                    .into(),
                            ),
                        });
                    }
                    Ok(_) if secrets.delete(reference).is_ok() => "",
                    _ => " The replacement key was retained in the credential store because cleanup could not be confirmed.",
                }
            } else {
                ""
            };
            Err(format!("{error}. The previous key was preserved and the form is available to retry.{suffix}"))
        }
    }
}

fn delete_connection(store: &SharedStore, id: &ConnectionId, secrets: &dyn SecretStore) -> Result<ConnectionChange, String> {
    let mut previous = None;
    let saved = store
        .update(|stored| {
            if let Some(Connection { auth: Auth::ApiKey { secret }, .. }) = stored.connections.iter().find(|connection| &connection.id == id) {
                previous = Some(secret.clone());
            }
            stored.connections.retain(|connection| &connection.id != id);
            // Preserve unavailable bindings so deleting a connection can never select a fallback.
            Ok(())
        })
        .map_err(fail)?;
    Ok(ConnectionChange { warning: cleanup_key(store, secrets, previous.as_ref()), store: saved })
}

fn cleanup_key(store: &SharedStore, secrets: &dyn SecretStore, reference: Option<&SecretRef>) -> Option<String> {
    let reference = reference?;
    // Check references while holding the shared settings lock. A key shared by another saved
    // connection remains valid; only an unreferenced API credential may be removed.
    store
        .update(|stored| {
            if !stored.connections.iter().any(|connection| connection.auth.secret_ref() == Some(reference)) {
                secrets.delete(reference)?;
            }
            Ok(())
        })
        .err()
        .map(|_| "Settings were saved, but an unused old API key could not be removed from the system credential store.".into())
}

struct DraftSecrets {
    temporary: Option<(SecretRef, SecretString)>,
}
impl SecretStore for DraftSecrets {
    fn get(&self, reference: &SecretRef) -> Result<SecretString, AiError> {
        if let Some((key, value)) = &self.temporary
            && key == reference
        {
            return Ok(value.clone());
        }
        KeyringStore.get(reference)
    }
    fn set(&self, _: &SecretRef, _: &SecretString) -> Result<(), AiError> {
        Err(AiError::invalid("a connection test never stores credentials"))
    }
    fn delete(&self, _: &SecretRef) -> Result<(), AiError> {
        Err(AiError::invalid("a connection test never deletes credentials"))
    }
}

fn discover(store: &SharedStore, mut draft: ConnectionDraft, verify: Option<String>, cancel: &Cancellation) -> Result<Update, String> {
    let prepared = draft.prepare(&store.load().map_err(fail)?.connections).map_err(fail)?;
    let temporary = match prepared.key {
        KeyChange::Set(value) => prepared.connection.auth.secret_ref().map(|reference| (reference.clone(), value)),
        _ => None,
    };
    let providers = Providers::new(store.root(), Arc::new(DraftSecrets { temporary }));
    let config = store.config().map_err(fail)?;
    let models = if let Some(model) = verify {
        let id = ModelId::new(model).map_err(fail)?;
        let info = craft_ai_native::runtime::verify_model(&providers, &prepared.connection, &id, &config, cancel).map_err(fail)?;
        let mut models = draft.models.clone();
        models.retain(|m| m.id != info.id);
        models.push(info);
        models
    } else {
        providers.models(&prepared.connection, &config, cancel).map_err(fail)?
    };
    let endpoint = draft.endpoint.clone();
    if !draft.set_models(&endpoint, models.clone()) {
        return Err("The connection changed while its models were loading; try again.".into());
    }
    Ok(Update::Models { draft, models })
}

fn codex_connection() -> Result<Connection, String> {
    Ok(Connection {
        id: ConnectionId::new("craft-codex").map_err(fail)?,
        label: "Codex account".into(),
        provider: "codex".into(),
        endpoint: None,
        locality: craft_ai_core::Locality::Remote,
        auth: Auth::Subscription { route: "codex".into(), secret: SecretRef::new("craft-codex-account").map_err(fail)? },
        billing: craft_ai_core::Billing::SubscriptionAllowance,
        models: Vec::new(),
        enabled: true,
        key_header: None,
    })
}

fn refresh_codex(store: &SharedStore, cancel: &Cancellation) -> Result<Update, String> {
    let provider = CodexProvider::new(CodexOptions::in_shared_root(store.root()));
    let config = store.config().map_err(fail)?;
    if !provider.signed_in(&config, cancel).map_err(fail)? {
        return Ok(Update::Notice("Sign into the Craft Codex account first.".into()));
    }
    provider.verify_isolation(&config, cancel).map_err(fail)?;
    let mut connection = codex_connection()?;
    connection.models = provider.models(&connection, &config, cancel).map_err(fail)?;
    save_connection(store, Prepared { connection, key: KeyChange::Keep }, &KeyringStore).map(|change| Update::Store(change.store))
}

fn login(store: &SharedStore, sender: &mpsc::Sender<Result<Update, String>>, cancel: &Cancellation) -> Result<Update, String> {
    let provider = CodexProvider::new(CodexOptions::in_shared_root(store.root()));
    let config = store.config().map_err(fail)?;
    let mut session = provider.start_login(&config, cancel).map_err(fail)?;
    let _ = sender.send(Ok(Update::Login { url: session.authorization_url.clone(), code: session.user_code.clone() }));
    loop {
        cancel.check().map_err(fail)?;
        match session.poll(cancel).map_err(fail)? {
            Some(true) => return refresh_codex(store, cancel),
            Some(false) => return Err("Codex sign-in was not completed".into()),
            None => std::thread::sleep(Duration::from_millis(100)),
        }
    }
}

pub(super) fn poll(app: &mut WordApp, ctx: &egui::Context) {
    poll_job(app, ctx);
    if let Some(receiver) = app.assistant.native.chooser.take() {
        match receiver.try_recv() {
            Ok(Ok(paths)) => {
                for path in paths {
                    if let Err(error) = add_file(app, path) {
                        app.assistant.widget.error = Some(error);
                    }
                }
            }
            Ok(Err(error)) => app.assistant.widget.error = Some(error),
            Err(mpsc::TryRecvError::Empty) => {
                app.assistant.native.chooser = Some(receiver);
                ctx.request_repaint_after(Duration::from_millis(100));
            }
            Err(mpsc::TryRecvError::Disconnected) => {}
        }
    }
    let Some(mut turn) = app.assistant.native.turn.take() else { return };
    while let Ok(pending) = turn.calls.try_recv() {
        host::dispatch(app, ctx, &mut turn.guard, pending);
    }
    let mut disconnected = false;
    loop {
        match turn.events.try_recv() {
            Ok(WorkerEvent::Runtime(event)) => apply_event(app, &mut turn, event),
            Ok(WorkerEvent::Saved(conversation)) => set_chat(app, conversation),
            Err(mpsc::TryRecvError::Empty) => break,
            Err(mpsc::TryRecvError::Disconnected) => {
                disconnected = true;
                break;
            }
        }
    }
    if disconnected {
        app.assistant.widget.busy = false;
    } else {
        app.assistant.native.turn = Some(turn);
        ctx.request_repaint_after(Duration::from_millis(50));
    }
}

fn apply_event(app: &mut WordApp, turn: &mut Turn, event: TurnEvent) {
    let state = &mut app.assistant.widget;
    match event {
        TurnEvent::Routed { role, connection, model } => {
            let route = format!("{connection} / {model}");
            match role {
                AssistantRole::Chat => state.chat_route = route.clone(),
                AssistantRole::Control => state.control_route = route.clone(),
            }
            state.status = Some(route);
            turn.fresh_text = true;
        }
        TurnEvent::Text { text, .. } => {
            if turn.fresh_text {
                state.messages.push(Message::text(MessageRole::Assistant, ""));
                turn.fresh_text = false;
            }
            if let Some(message) = state.messages.last_mut() {
                message.content.push_str(&text);
            }
        }
        TurnEvent::ToolStarted { call } => {
            turn.pending_activity.push((call.id, state.activity.len()));
            state.activity.push(Activity { label: call.name, detail: tl!("Running…").into(), failed: false });
        }
        TurnEvent::ToolFinished { id, result } => {
            if let Some(position) = turn.pending_activity.iter().rposition(|(pending, _)| pending == &id) {
                let (_, index) = turn.pending_activity.remove(position);
                if let Some(activity) = state.activity.get_mut(index) {
                    activity.detail = result.to_string();
                    activity.failed = result.get("isError").and_then(Value::as_bool) == Some(true);
                }
            }
        }
        TurnEvent::Checkpoint { .. } => {}
        TurnEvent::Finished { messages } => {
            state.messages = messages;
            state.status = Some(tl!("Complete").into());
        }
        TurnEvent::Failed { message } => {
            state.error = Some(format!("{message}. {}", tl!("Completed edits remain undoable.")));
            for (_, index) in turn.pending_activity.drain(..) {
                if let Some(activity) = state.activity.get_mut(index) {
                    activity.detail = tl!("Interrupted; inspect the document before requesting more changes.").into();
                    activity.failed = true;
                }
            }
            state.status = None;
        }
    }
}

fn set_chat(app: &mut WordApp, conversation: Conversation) {
    let state = &mut app.assistant.widget;
    state.messages = conversation.messages.clone();
    state.active_chat = Some(conversation.id.clone());
    state.history.retain(|c| c.id != conversation.id);
    state.history.insert(0, ChatSummary { id: conversation.id.clone(), title: conversation.title.clone(), interrupted: conversation.interrupted });
    state.status = conversation.interrupted.then(|| tl!("Interrupted conversation. Inspect the document before requesting further edits.").into());
    app.assistant.native.conversation = Some(conversation);
}

fn set_store(app: &mut WordApp, store: StoreFile) {
    if let Ok(id) = app_id() {
        let route = |role: AssistantRole| -> String {
            let selected = role.declaration().map_err(fail).and_then(|declaration| {
                store.resolve(&id, &declaration).map(|resolved| format!("{} / {}", resolved.connection.label, resolved.model.id)).map_err(fail)
            });
            selected.unwrap_or_else(|error| format!("{}: {error}", tl!("Unavailable")))
        };
        app.assistant.widget.chat_route = route(AssistantRole::Chat);
        app.assistant.widget.control_route = route(AssistantRole::Control);
        app.assistant.widget.settings.codex_status = if store.connections.iter().any(|connection| connection.provider == "codex") {
            tl!("Codex connection saved.").into()
        } else {
            String::new()
        };
        app.assistant.widget.settings.set_store(store, Some(id));
    }
}

fn poll_job(app: &mut WordApp, ctx: &egui::Context) {
    let Some(receiver) = app.assistant.native.job.take() else { return };
    loop {
        match receiver.try_recv() {
            Ok(Ok(update)) => match update {
                Update::Loaded { root, store, history } => {
                    app.assistant.native.root = Some(root);
                    app.assistant.widget.history = history;
                    set_store(app, store);
                }
                Update::Store(store) => {
                    set_store(app, store);
                    app.assistant.widget.settings.status = Some(tl!("Settings saved.").into());
                }
                Update::ConnectionSaved { change, id } => {
                    let draft = change.store.connections.iter().find(|connection| connection.id == id).map(ConnectionDraft::from_connection);
                    set_store(app, change.store);
                    if let Some(draft) = draft {
                        app.assistant.widget.settings.connection = draft;
                    }
                    app.assistant.widget.settings.status = Some(tl!("Connection saved.").into());
                    app.assistant.widget.settings.error = change.warning;
                }
                Update::ConnectionDeleted { change, id } => {
                    set_store(app, change.store);
                    if app.assistant.widget.settings.connection.editing.as_ref() == Some(&id) {
                        app.assistant.widget.settings.connection = ConnectionDraft::default();
                        app.assistant.widget.settings.preset.clear();
                    }
                    app.assistant.widget.settings.status = Some(tl!("Connection deleted.").into());
                    app.assistant.widget.settings.error = change.warning;
                }
                Update::Models { draft, models } => {
                    let current = &mut app.assistant.widget.settings.connection;
                    if current.provider == draft.provider && current.editing == draft.editing && current.set_models(&draft.endpoint, models) {
                        app.assistant.widget.settings.status = Some(tl!("Model list updated. Save the connection to keep it.").into());
                    }
                }
                Update::Chat(chat) => {
                    app.assistant.widget.activity.clear();
                    app.assistant.widget.error = None;
                    set_chat(app, chat);
                }
                Update::Deleted(id) => {
                    app.assistant.widget.history.retain(|c| c.id != id);
                    if app.assistant.widget.active_chat.as_ref() == Some(&id) {
                        app.assistant.widget.active_chat = None;
                        app.assistant.widget.messages.clear();
                        app.assistant.native.conversation = None;
                    }
                }
                Update::Login { url, code } => {
                    app.assistant.widget.settings.codex_status = format!("{} {code}", tl!("Complete sign-in in your browser. Code:"));
                    ctx.open_url(egui::OpenUrl::new_tab(url));
                }
                Update::Notice(message) => app.assistant.widget.settings.status = Some(message),
            },
            Ok(Err(error)) => {
                app.assistant.widget.settings.codex_status.clear();
                app.assistant.widget.error = Some(error.clone());
                app.assistant.widget.settings.error = Some(error);
            }
            Err(mpsc::TryRecvError::Empty) => {
                app.assistant.native.job = Some(receiver);
                ctx.request_repaint_after(Duration::from_millis(100));
                break;
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                app.assistant.widget.settings.busy = false;
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::BTreeMap;
    use std::sync::Mutex;

    struct Temp(PathBuf);
    impl Temp {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "wordcraft-native-assistant-{}-{}",
                std::process::id(),
                NEXT_SECRET.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir_all(&path).unwrap();
            Self(path)
        }
        fn store(&self) -> SharedStore {
            SharedStore::new(self.0.clone())
        }
    }
    impl Drop for Temp {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[derive(Default)]
    struct Secrets {
        values: Mutex<BTreeMap<SecretRef, SecretString>>,
        fail_set: bool,
        fail_delete: bool,
    }
    impl SecretStore for Secrets {
        fn get(&self, reference: &SecretRef) -> Result<SecretString, AiError> {
            self.values.lock().unwrap().get(reference).cloned().ok_or_else(|| AiError::invalid("missing test credential"))
        }
        fn set(&self, reference: &SecretRef, value: &SecretString) -> Result<(), AiError> {
            if self.fail_set {
                return Err(AiError::invalid("untrusted error with synthetic-private-key"));
            }
            self.values.lock().unwrap().insert(reference.clone(), value.clone());
            Ok(())
        }
        fn delete(&self, reference: &SecretRef) -> Result<(), AiError> {
            if self.fail_delete {
                return Err(AiError::invalid("untrusted error with synthetic-private-key"));
            }
            self.values.lock().unwrap().remove(reference);
            Ok(())
        }
    }
    fn draft() -> ConnectionDraft {
        let mut draft = ConnectionDraft::default();
        draft.label = "Synthetic endpoint".into();
        draft.endpoint = "https://example.com/v1".into();
        draft.key = "synthetic-private-key".into();
        draft
    }
    fn save(store: &SharedStore, secrets: &Secrets) -> Connection {
        save_connection(store, draft().prepare(&[]).unwrap(), secrets).unwrap().store.connections.remove(0)
    }
    fn app() -> WordApp {
        WordApp::new(wordcraft_engine::Session::new(wordcraft_doc::Document::from_text("Synthetic document")), crate::Services::default())
    }

    #[test]
    fn replaced_keys_are_staged_then_old_keys_removed_without_plaintext_settings() {
        let temp = Temp::new();
        let store = temp.store();
        let secrets = Secrets::default();
        let original = save(&store, &secrets);
        let mut edit = ConnectionDraft::from_connection(&original);
        edit.key = "synthetic-replacement-key".into();
        let changed = save_connection(&store, edit.prepare(std::slice::from_ref(&original)).unwrap(), &secrets).unwrap();
        let replacement = changed.store.connections.first().unwrap();
        assert_ne!(replacement.auth.secret_ref(), original.auth.secret_ref());
        assert!(secrets.get(original.auth.secret_ref().unwrap()).is_err());
        assert_eq!(secrets.get(replacement.auth.secret_ref().unwrap()).unwrap().expose(), "synthetic-replacement-key");
        let contents = std::fs::read_to_string(temp.0.join("connections.json")).unwrap();
        assert!(!contents.contains("synthetic-private-key"));
        assert!(!contents.contains("synthetic-replacement-key"));
        assert!(changed.warning.is_none());
    }

    #[test]
    fn credential_failure_preserves_saved_state_and_redacts_untrusted_error() {
        let temp = Temp::new();
        let store = temp.store();
        let mut secrets = Secrets::default();
        let original = save(&store, &secrets);
        secrets.fail_set = true;
        let error = save_connection(&store, draft().prepare(std::slice::from_ref(&original)).unwrap(), &secrets).err().unwrap();
        assert!(!error.contains("synthetic-private-key"));
        assert_eq!(store.load().unwrap().connections, vec![original.clone()]);
        assert_eq!(secrets.get(original.auth.secret_ref().unwrap()).unwrap().expose(), "synthetic-private-key");
    }

    #[test]
    fn failed_settings_write_never_destroys_the_previous_credential() {
        let temp = Temp::new();
        let store = temp.store();
        let secrets = Secrets::default();
        let original = save(&store, &secrets);
        std::fs::write(temp.0.join("connections.json"), "corrupt synthetic fixture").unwrap();
        let mut edit = ConnectionDraft::from_connection(&original);
        edit.key = "synthetic-new-key".into();
        let error = save_connection(&store, edit.prepare(std::slice::from_ref(&original)).unwrap(), &secrets).err().unwrap();
        assert!(!error.contains("synthetic-new-key"));
        assert_eq!(secrets.get(original.auth.secret_ref().unwrap()).unwrap().expose(), "synthetic-private-key");
    }

    #[test]
    fn clear_and_delete_remove_only_unreferenced_keys() {
        let temp = Temp::new();
        let store = temp.store();
        let secrets = Secrets::default();
        let original = save(&store, &secrets);
        let mut edit = ConnectionDraft::from_connection(&original);
        edit.uses_key = false;
        let cleared = save_connection(&store, edit.prepare(std::slice::from_ref(&original)).unwrap(), &secrets).unwrap();
        assert!(cleared.store.connections.first().unwrap().auth.secret_ref().is_none());
        assert!(secrets.get(original.auth.secret_ref().unwrap()).is_err());
        let first = save(&store, &secrets);
        let mut second = first.clone();
        second.id = ConnectionId::new("second").unwrap();
        store
            .update(|stored| {
                stored.connections.push(second.clone());
                Ok(())
            })
            .unwrap();
        delete_connection(&store, &first.id, &secrets).unwrap();
        assert!(secrets.get(first.auth.secret_ref().unwrap()).is_ok());
        delete_connection(&store, &second.id, &secrets).unwrap();
        assert!(secrets.get(first.auth.secret_ref().unwrap()).is_err());
    }

    #[test]
    fn cleanup_failure_is_a_warning_after_the_connection_was_saved() {
        let temp = Temp::new();
        let store = temp.store();
        let mut secrets = Secrets::default();
        let original = save(&store, &secrets);
        secrets.fail_delete = true;
        let mut edit = ConnectionDraft::from_connection(&original);
        edit.uses_key = false;
        let result = save_connection(&store, edit.prepare(std::slice::from_ref(&original)).unwrap(), &secrets).unwrap();
        assert!(result.store.connections.first().unwrap().auth.secret_ref().is_none());
        let warning = result.warning.unwrap();
        assert!(!warning.contains("synthetic-private-key"));
        assert!(warning.contains("saved"));
    }

    #[test]
    fn stale_empty_key_edit_keeps_the_latest_saved_credential() {
        let temp = Temp::new();
        let store = temp.store();
        let secrets = Secrets::default();
        let original = save(&store, &secrets);
        let stale = ConnectionDraft::from_connection(&original);
        let mut replacement = stale.clone();
        replacement.key = "new-synthetic-key".into();
        let saved = save_connection(&store, replacement.prepare(std::slice::from_ref(&original)).unwrap(), &secrets).unwrap();
        let current = saved.store.connections.first().unwrap().auth.secret_ref().unwrap();
        let updated = save_connection(&store, stale.prepare(std::slice::from_ref(&original)).unwrap(), &secrets).unwrap();
        assert_eq!(updated.store.connections.first().unwrap().auth.secret_ref(), Some(current));
        assert_eq!(secrets.get(current).unwrap().expose(), "new-synthetic-key");
    }

    #[test]
    fn checkpoints_persist_completed_commands_and_cancel_on_write_failure() {
        let temp = Temp::new();
        let history = history(&temp.store(), &app_id().unwrap()).unwrap();
        let mut saved = history.create("Synthetic history").unwrap();
        let mut call = Message::text(MessageRole::Assistant, "");
        call.tool_calls.push(craft_ai_core::assistant::ToolCall {
            id: "delegation".into(),
            name: "control_app".into(),
            arguments: json!({"task":"edit"}),
        });
        let messages = vec![
            Message::text(MessageRole::User, "Edit"),
            call,
            Message::tool("delegation", &json!({"activity":[{"id":"edit-1","outcome":"completed"}]})),
        ];
        let cancel = Cancellation::new();
        save_checkpoint(&history, &mut saved, &messages, &cancel).unwrap();
        let recovered = history.load(&saved.id).unwrap();
        assert!(recovered.interrupted);
        assert_eq!(recovered.messages, messages);
        assert!(!cancel.is_cancelled());
        let file = temp.0.join("chats/wordcraft").join(format!("{}.json", saved.id));
        std::fs::remove_file(&file).unwrap();
        std::fs::create_dir(&file).unwrap();
        assert!(save_checkpoint(&history, &mut saved, &messages, &cancel).is_err());
        assert!(cancel.is_cancelled());
        assert_eq!(saved.messages, messages);
    }

    #[test]
    fn save_acknowledgment_keeps_connection_identity_and_failed_send_keeps_draft() {
        let mut app = app();
        app.assistant.enabled = true;
        app.assistant.native.initialized = true;
        assert!(action(&mut app, Action::Send("keep this unsent prompt".into())).is_err());
        assert_eq!(app.assistant.widget.draft, "keep this unsent prompt");
        let connection = draft().prepare(&[]).unwrap().connection;
        app.assistant.widget.settings.connection = draft();
        let (sender, receiver) = mpsc::channel();
        sender
            .send(Ok(Update::ConnectionSaved {
                change: ConnectionChange { store: StoreFile { connections: vec![connection.clone()], ..Default::default() }, warning: None },
                id: connection.id.clone(),
            }))
            .unwrap();
        drop(sender);
        app.assistant.native.job = Some(receiver);
        poll_job(&mut app, &egui::Context::default());
        assert_eq!(app.assistant.widget.settings.connection.editing, Some(connection.id));
        assert!(app.assistant.widget.settings.connection.key.is_empty());
    }

    #[test]
    fn nested_tool_results_update_their_own_activity_and_removed_routes_are_visible() {
        let mut app = app();
        let mut turn = Turn {
            events: mpsc::channel().1,
            calls: mpsc::channel().1,
            cancel: Cancellation::new(),
            guard: Guard::new(&app, "test".into(), vec![]),
            fresh_text: true,
            pending_activity: Vec::new(),
        };
        for name in ["control_app", "type_text"] {
            apply_event(
                &mut app,
                &mut turn,
                TurnEvent::ToolStarted { call: craft_ai_core::assistant::ToolCall { id: "same-id".into(), name: name.into(), arguments: json!({}) } },
            );
        }
        apply_event(&mut app, &mut turn, TurnEvent::ToolFinished { id: "same-id".into(), result: json!({"edit":"done"}) });
        apply_event(&mut app, &mut turn, TurnEvent::ToolFinished { id: "same-id".into(), result: json!({"delegation":"done"}) });
        assert!(app.assistant.widget.activity.first().unwrap().detail.contains("delegation"));
        assert!(app.assistant.widget.activity.get(1).unwrap().detail.contains("edit"));
        app.assistant.widget.chat_route = "stale provider".into();
        set_store(&mut app, StoreFile::default());
        assert!(!app.assistant.widget.chat_route.contains("stale provider"));
        assert!(app.assistant.widget.chat_route.contains("Unavailable"));
    }

    /// Decode actual MCP read results, never model summaries or mutation result echoes.
    fn live_mcp_read_results(call: &craft_ai_core::assistant::ToolCall, result: &Value) -> Vec<String> {
        fn read_body(command: &str, value: &Value) -> Option<String> {
            match command {
                "document.text" => value.get("text")?.as_str().map(str::to_owned),
                "document.inspect" => value
                    .get("blocks")?
                    .as_array()?
                    .iter()
                    .map(|block| {
                        (block.get("type")?.as_str()? == "paragraph").then_some(())?;
                        block.get("text")?.as_str()
                    })
                    .collect::<Option<Vec<_>>>()
                    .map(|paragraphs| paragraphs.join("\n")),
                _ => None,
            }
        }
        let mut reads = Vec::new();
        for content in result.get("content").and_then(Value::as_array).into_iter().flatten() {
            if content.get("type").and_then(Value::as_str) != Some("text") {
                continue;
            }
            let Some(body) = content.get("text").and_then(Value::as_str).and_then(|text| serde_json::from_str::<Value>(text).ok()) else { continue };
            if call.name == "batch" {
                let Some(requested) = call.arguments.get("commands").and_then(Value::as_array) else { continue };
                let Some(results) = body.as_array() else { continue };
                // A failed batch may still contain successful earlier commands. Preserve order.
                for (request, item) in requested.iter().zip(results) {
                    let Some(command) = request.get("command").and_then(Value::as_str) else { continue };
                    if item.get("command").and_then(Value::as_str) == Some(command)
                        && item.get("ok").and_then(Value::as_bool) == Some(true)
                        && let Some(text) = item.get("result").and_then(|value| read_body(command, value))
                    {
                        reads.push(text);
                    }
                }
            } else if result.get("isError").and_then(Value::as_bool) != Some(true) {
                let command = match call.name.as_str() {
                    "get_text" => Some("document.text"),
                    "inspect_document" => Some("document.inspect"),
                    "execute" => call.arguments.get("command").and_then(Value::as_str),
                    _ => None,
                };
                if let Some(text) = command.and_then(|command| read_body(command, &body)) {
                    reads.push(text);
                }
            }
        }
        reads
    }

    #[test]
    fn live_read_observer_accepts_only_successful_mcp_read_results() {
        use craft_ai_core::assistant::ToolCall;
        use wordcraft_mcp::tools::ToolResult;
        let call = |name: &str, arguments| ToolCall { id: "synthetic".into(), name: name.into(), arguments };
        let text = ToolResult::json(&json!({"text":"before"})).to_value();
        let inspect = ToolResult::json(&json!({"blocks":[{"type":"paragraph","text":"before"}],"selection":{"text":"ignore selection"}})).to_value();
        assert_eq!(live_mcp_read_results(&call("get_text", json!({})), &text), ["before"]);
        assert_eq!(live_mcp_read_results(&call("inspect_document", json!({})), &inspect), ["before"]);
        assert_eq!(live_mcp_read_results(&call("execute", json!({"command":"document.text"})), &text), ["before"]);
        assert_eq!(live_mcp_read_results(&call("execute", json!({"command":"document.inspect"})), &inspect), ["before"]);
        for name in ["control_app", "type_text"] {
            assert!(live_mcp_read_results(&call(name, json!({})), &text).is_empty());
        }
        assert!(live_mcp_read_results(&call("execute", json!({"command":"text.insert"})), &text).is_empty());
        let batch = call(
            "batch",
            json!({"commands":[
                {"command":"document.text"}, {"command":"text.insert"}, {"command":"document.inspect"}, {"command":"document.text"}
            ]}),
        );
        let mut result = ToolResult::json(&json!([
            {"command":"document.text","ok":true,"result":{"text":"before"}},
            {"command":"text.insert","ok":true,"result":{"text":"not a read"}},
            {"command":"document.inspect","ok":true,"result":{"blocks":[{"type":"paragraph","text":"after"}]}},
            {"command":"document.text","ok":false,"result":{"text":"failed"}}
        ]))
        .to_value();
        result["isError"] = json!(true);
        assert_eq!(live_mcp_read_results(&batch, &result), ["before", "after"]);
        let mut failed = text.clone();
        failed["isError"] = json!(true);
        assert!(live_mcp_read_results(&call("get_text", json!({})), &failed).is_empty());
        let mismatch = call("batch", json!({"commands":[{"command":"text.insert"}]}));
        assert!(live_mcp_read_results(&mismatch, &result).is_empty());
        assert!(live_mcp_read_results(&call("get_text", json!({})), &ToolResult::text("{bad json").to_value()).is_empty());
    }

    /// Opt in with an isolated, explicitly configured CRAFT_AI_CONFIG_DIR. Both role bindings
    /// and runtime.json must already exist; credentials/sign-in are prepared outside this test.
    /// This sends only synthetic text, opens no listener and writes history only under Temp.
    #[test]
    #[ignore = "live provider acceptance requires an explicit isolated CRAFT_AI_CONFIG_DIR"]
    fn live_configured_models_edit_read_back_and_undo_through_mcp() {
        use craft_ai_native::runtime::verify_model;
        use std::time::Instant;

        // Native adapters redact credentials and do not put HTTP response bodies in AiError.
        // Keep diagnostics single-line and bounded; never print prompts, arguments or results.
        fn diagnostic(message: &str) -> String {
            message.chars().filter(|ch| !ch.is_control()).take(512).collect()
        }

        const EXPECTED: &str = "Craft live assistant acceptance completed.";
        let root = PathBuf::from(std::env::var_os("CRAFT_AI_CONFIG_DIR").expect("set an isolated CRAFT_AI_CONFIG_DIR for this live test"));
        assert!(root.is_absolute(), "live test configuration must be an explicit absolute directory");
        assert!(root.join("connections.json").is_file(), "prepare connections.json in the selected live test directory");
        assert!(root.join("runtime.json").is_file(), "persist runtime limits in the selected live test directory");
        let shared = SharedStore::new(root.clone());
        let store = shared.load().unwrap_or_else(|_| panic!("the explicit live test connection store could not be loaded"));
        let config = shared.config().unwrap_or_else(|_| panic!("the explicit live test runtime limits could not be loaded"));
        let app_id = app_id().unwrap();
        let (chat, control) = resolve_choices(&store, &app_id).unwrap_or_else(|_| panic!("configure both WordCraft assistant role bindings"));
        let cancel = Cancellation::new();
        for (index, choice) in [&chat, &control].into_iter().enumerate() {
            if index == 1 && choice.connection.id == chat.connection.id && choice.model == chat.model {
                continue;
            }
            // A standalone capability probe leaves its callback pending until its provider drops.
            let probe = Providers::new(&root, Arc::new(KeyringStore));
            if let Err(error) = verify_model(&probe, &choice.connection, &choice.model, &config, &cancel) {
                panic!("configured model failed the synthetic tool-capability probe ({:?}): {}", error.kind, diagnostic(&error.message));
            }
        }
        let providers = Providers::new(&root, Arc::new(KeyringStore));

        let mut app = app();
        enable_without_profile(&mut app);
        let original = app.session.doc.plain_text(wordcraft_doc::StoryRef::Body);
        let prompt = format!(
            "Perform this synthetic WordCraft acceptance task on the current document. First use get_text to read it. \
             Replace its entire body with exactly this single line, without quotes or a trailing newline: {EXPECTED} \
             Use execute with select.all, followed by one type_text call for the replacement, so the edit is one Undo step. \
             Then use get_text again to verify the exact result. Do not undo it; the test will exercise Undo afterward. \
             Do not open, create, save, or read any files, change settings, or add any other content."
        );
        let target = Target { id: app.session.document_id().to_string(), revision: app.session.rev() };
        let mut guard = Guard::new(&app, prompt.clone(), vec![]);
        let ctx = egui::Context::default();
        let wake = ctx.clone();
        let (sender, requests) = mpsc::channel();
        let worker_cancel = cancel.clone();
        let timeout = Duration::from_secs(config.request_timeout_secs);
        let deadline = Instant::now()
            .checked_add(Duration::from_secs(config.request_timeout_secs.saturating_mul(u64::from(config.max_rounds))))
            .expect("validated live test limits must fit the monotonic clock");
        let temporary = Temp::new();
        let history = history(&temporary.store(), &app_id).unwrap();
        let mut saved = history.create("Synthetic live assistant acceptance").unwrap();
        let messages = vec![Message::text(MessageRole::User, prompt)];
        saved.messages = messages.clone();
        saved.interrupted = true;
        history.save(&saved).unwrap();
        let shared_choice = chat.connection == control.connection && chat.model == control.model;
        let request = TurnRequest { app: app_id, messages, target, chat, control, config };
        let expected_original = original.clone();
        let worker = std::thread::spawn(move || {
            let mut host = McpHost::new(sender, worker_cancel.clone(), wake, timeout);
            let mut pending_calls = BTreeMap::<String, Vec<craft_ai_core::assistant::ToolCall>>::new();
            let mut pending_names = BTreeMap::new();
            let mut tool_trace = Vec::new();
            let (mut tools_started, mut tools_finished) = (0_usize, 0_usize);
            let (mut chat_used, mut control_used, mut read_before, mut read_after) = (false, false, false, false);
            let mut persistence_failed = false;
            let outcome = run_turn(request, &providers, &mut host, &worker_cancel, |event| match event {
                TurnEvent::Routed { role: AssistantRole::Chat, .. } => chat_used = true,
                TurnEvent::Routed { role: AssistantRole::Control, .. } => control_used = true,
                TurnEvent::ToolStarted { call } => {
                    tools_started += 1;
                    let name = [
                        "control_app",
                        "list_commands",
                        "execute",
                        "batch",
                        "open_document",
                        "save_document",
                        "type_text",
                        "select_text",
                        "get_text",
                        "inspect_document",
                    ]
                    .into_iter()
                    .find(|name| *name == call.name)
                    .unwrap_or("unrecognized");
                    if tool_trace.len() < 32 {
                        tool_trace.push(format!("{name}:started"));
                    }
                    pending_names.insert(call.id.clone(), name);
                    pending_calls.entry(call.id.clone()).or_default().push(call);
                }
                TurnEvent::ToolFinished { id, result } => {
                    tools_finished += 1;
                    let name = pending_names.remove(&id).unwrap_or("unmatched");
                    let failed = result.get("isError").and_then(Value::as_bool) == Some(true);
                    if tool_trace.len() < 32 {
                        tool_trace.push(format!("{name}:{}", if failed { "error" } else { "ok" }));
                    }
                    if let Some(call) = pending_calls.get_mut(&id).and_then(Vec::pop) {
                        for text in live_mcp_read_results(&call, &result) {
                            read_before |= text == expected_original;
                            read_after |= read_before && text == EXPECTED;
                        }
                    }
                }
                TurnEvent::Checkpoint { messages } => {
                    persistence_failed |= save_checkpoint(&history, &mut saved, &messages, &worker_cancel).is_err();
                }
                _ => {}
            })
            .map(|_| ())
            .map_err(|error| (error.kind, diagnostic(&error.message)));
            (outcome, chat_used, control_used, read_before, read_after, persistence_failed, tools_started, tools_finished, tool_trace)
        });
        let mut expired = false;
        loop {
            match requests.recv_timeout(Duration::from_millis(25)) {
                Ok(pending) => host::dispatch(&mut app, &ctx, &mut guard, pending),
                Err(mpsc::RecvTimeoutError::Disconnected) => break,
                Err(mpsc::RecvTimeoutError::Timeout) if Instant::now() >= deadline => {
                    expired = true;
                    cancel.cancel();
                    break;
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {}
            }
        }
        drop(requests);
        let (outcome, chat_used, control_used, read_before, read_after, persistence_failed, tools_started, tools_finished, tool_trace) =
            worker.join().unwrap_or_else(|_| panic!("the live assistant worker failed"));
        eprintln!(
            "live acceptance summary: turn_ok={} expired={expired} persistence_failed={persistence_failed} \
             chat={chat_used} control={control_used} read_before={read_before} read_after={read_after} \
             tools_started={tools_started} tools_finished={tools_finished} document_changed={} \
             document_matches_expected={} tool_trace={tool_trace:?}",
            outcome.is_ok(),
            app.session.doc.plain_text(wordcraft_doc::StoryRef::Body) != original,
            app.session.doc.plain_text(wordcraft_doc::StoryRef::Body) == EXPECTED,
        );
        assert!(!expired, "the live turn exceeded its configured round/time budget");
        assert!(!persistence_failed, "synthetic recovery history could not be persisted");
        if let Err((kind, message)) = outcome {
            panic!("configured live turn failed ({kind:?}): {message}");
        }
        assert!(chat_used && (control_used || shared_choice), "live acceptance must use each distinct selected model route");
        assert!(read_before && read_after, "the control model must read the original and verify its edit through MCP");
        assert!(app.session.doc.plain_text(wordcraft_doc::StoryRef::Body) == EXPECTED, "the synthetic document differs from the requested result");
        assert!(app.session.can_undo(), "the assistant edit must be in the normal Undo history");
        assert!(app.session.run("edit.undo", &json!({})).is_ok(), "normal Undo failed");
        assert!(app.session.doc.plain_text(wordcraft_doc::StoryRef::Body) == original, "one normal Undo must restore the original synthetic text");
    }
}
