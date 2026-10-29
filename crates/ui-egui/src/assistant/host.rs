//! The assistant uses the same MCP server as external clients, with a per-turn app boundary.

use std::sync::{Arc, mpsc};
use std::time::{Duration, Instant};

use craft_ai_core::assistant::{Target, ToolCall, ToolDefinition};
use craft_ai_core::{AiError, ErrorKind};
use craft_ai_native::{Cancellation, ToolHost};
use serde_json::{Value, json};
use wordcraft_engine::file_access::{FileGrants, confine_files};
use wordcraft_mcp::{backend::Backend, server::Server};

use crate::{WordApp, control};

pub(super) struct Pending {
    method: String,
    params: Value,
    reply: mpsc::Sender<Result<Value, String>>,
    cancel: Cancellation,
    expires: Instant,
}

pub(super) struct Guard {
    document: u64,
    window: u64,
    had_document: bool,
    revision: u64,
    selection: SelectionState,
    grants: Result<Arc<FileGrants>, String>,
    read: bool,
    visual_read: bool,
}

#[derive(PartialEq)]
struct SelectionState {
    text: wordcraft_engine::Selection,
    ends: wordcraft_engine::Selection,
    objects: Vec<wordcraft_doc::Pos>,
    column: Option<wordcraft_engine::ColumnBlock>,
    math: Option<wordcraft_engine::MathEdit>,
}

impl SelectionState {
    fn new(session: &wordcraft_engine::Session) -> Self {
        Self {
            text: session.sel.clone(),
            ends: session.sel_ends().clone(),
            objects: session.also_selected.clone(),
            column: session.column.clone(),
            math: session.math.clone(),
        }
    }
}

impl Guard {
    pub(super) fn new(app: &WordApp, prompt: String, mut files: Vec<String>) -> Self {
        files.extend(named_files(&prompt));
        files.sort();
        files.dedup();
        Self {
            document: app.session.document_id(),
            window: app.windows.shown(),
            had_document: app.has_document(),
            revision: app.session.rev(),
            selection: SelectionState::new(&app.session),
            grants: FileGrants::new(&files).map(Arc::new),
            read: false,
            visual_read: false,
        }
    }
}

/// Quoted paths may contain spaces. Unquoted paths must be complete whitespace-delimited tokens.
fn named_files(prompt: &str) -> Vec<String> {
    let mut paths = Vec::new();
    let mut token = String::new();
    let mut quote = None;
    let flush = |token: &mut String, paths: &mut Vec<String>| {
        if std::path::Path::new(token.as_str()).is_absolute() && paths.len() < 33 {
            paths.push(std::mem::take(token));
        } else {
            token.clear();
        }
    };
    for ch in prompt.chars() {
        if quote == Some(ch) {
            flush(&mut token, &mut paths);
            quote = None;
        } else if quote.is_some() {
            token.push(ch);
        } else if matches!(ch, '\'' | '"' | '`') && token.is_empty() {
            quote = Some(ch);
        } else if ch.is_whitespace() {
            flush(&mut token, &mut paths);
        } else {
            token.push(ch);
        }
    }
    if quote.is_none() {
        flush(&mut token, &mut paths);
    }
    paths
}

struct Channel {
    sender: mpsc::Sender<Pending>,
    cancel: Cancellation,
    wake: egui::Context,
    timeout: Duration,
}

impl Backend for Channel {
    fn call(&mut self, method: &str, params: Value) -> Result<Value, String> {
        self.cancel.check().map_err(|e| e.to_string())?;
        let (reply, receiver) = mpsc::channel();
        let expires = Instant::now().checked_add(self.timeout).ok_or("assistant timeout is too large")?;
        self.sender
            .send(Pending { method: method.into(), params, reply, cancel: self.cancel.clone(), expires })
            .map_err(|_| "the document was closed")?;
        self.wake.request_repaint();
        loop {
            self.cancel.check().map_err(|e| e.to_string())?;
            match receiver.recv_timeout(Duration::from_millis(50)) {
                Ok(result) => return result,
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    self.cancel.cancel();
                    return Err("the app stopped answering; the command outcome is unknown and will not be replayed".into());
                }
                Err(mpsc::RecvTimeoutError::Timeout) if Instant::now() >= expires => {
                    self.cancel.cancel();
                    return Err("the app did not answer; the command will not be replayed".into());
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {}
            }
        }
    }
    fn has_ui(&self) -> bool {
        false
    }
    fn describe(&self) -> String {
        "WordCraft assistant".into()
    }
}

pub(super) struct McpHost {
    server: Server,
}

impl McpHost {
    pub(super) fn new(sender: mpsc::Sender<Pending>, cancel: Cancellation, wake: egui::Context, timeout: Duration) -> Self {
        let mut server = Server::new(Box::new(Channel { sender, cancel, wake, timeout }));
        server.handle(json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"craft-assistant","version":"1"}}}));
        server.handle(json!({"jsonrpc":"2.0","method":"notifications/initialized"}));
        Self { server }
    }

    fn request(&mut self, method: &str, params: Value) -> Result<Value, AiError> {
        let reply = self
            .server
            .handle(json!({"jsonrpc":"2.0","id":2,"method":method,"params":params}))
            .ok_or_else(|| AiError::invalid("MCP did not answer"))?;
        if let Some(error) = reply.get("error") {
            return Err(AiError::new(ErrorKind::Provider, error.to_string()));
        }
        reply.get("result").cloned().ok_or_else(|| AiError::invalid("invalid MCP response"))
    }
}

const TOOLS: &[&str] = &[
    "list_commands",
    "execute",
    "batch",
    "new_document",
    "open_document",
    "list_documents",
    "save_document",
    "type_text",
    "select_text",
    "get_text",
    "inspect_document",
    "ui_inspect",
    "parity",
];

/// External MCP clients can address documents, but an assistant turn already has its target.
/// Adapt a copy of the catalog; supplied document IDs still pass through the dispatch guard.
fn bound_tool(tool: &Value) -> Option<ToolDefinition> {
    let name = tool.get("name")?.as_str()?;
    if !TOOLS.contains(&name) {
        return None;
    }
    let mut schema = tool.get("inputSchema")?.as_object()?.clone();
    schema.get_mut("properties")?.as_object_mut()?.remove("doc");
    if let Some(required) = schema.get_mut("required").and_then(Value::as_array_mut) {
        required.retain(|property| property.as_str() != Some("doc"));
    }
    schema.insert("additionalProperties".into(), Value::Bool(false));
    let description = match name {
        "execute" => {
            "Run a permitted command for operations other than text replacement. First call list_commands and use its exact command id and documented params; do not guess either. For page-display or interface-theme changes, call ui_inspect and supply an explicit value. To replace text, use select_text followed by type_text."
        }
        "batch" => {
            "Run permitted commands in order; stops at the first error unless keepGoing. First call list_commands and use its exact command ids and documented params; do not guess either. For page-display or interface-theme changes, call ui_inspect and supply explicit values. For text replacement, use select_text followed by type_text."
        }
        "select_text" => {
            "Select the n-th occurrence of exact existing text (occurrence is 1-based). To replace it, call this tool first, then type_text with only the replacement text. This replaces the selected text, preserves the rest of the document, and supports normal Undo."
        }
        "open_document" => {
            "Open a file the user explicitly named or chose. Supply its path and password if protected. After a successful open, subsequent tools operate on that document."
        }
        "new_document" => {
            "Create a document from a built-in template beside the open documents, preserving their writing. Subsequent tools operate on the created document; inspect it before editing."
        }
        "list_documents" => {
            "List open-document metadata without changing the active document or granting access to another document's content. To work on another file, the user must explicitly name or choose its path."
        }
        "ui_inspect" => {
            "Inspect only the bound window's dark-page, reading-page-color and interface-theme state. Returns the chosen interface theme and its effective light/dark appearance; does not inspect other preferences or documents."
        }
        _ => tool.get("description").and_then(Value::as_str).unwrap_or_default(),
    };
    let replacement = if name == "type_text" {
        " To replace existing text, first call select_text, then pass only the replacement text here. The selected text is replaced, the rest of the document is preserved, and one normal Undo restores the edit."
    } else {
        ""
    };
    Some(ToolDefinition {
        name: name.into(),
        description: format!(
            "{description}{replacement} WordCraft already scopes this tool to the document bound to the assistant turn. Pass only the listed arguments."
        ),
        input_schema: Value::Object(schema),
    })
}

impl ToolHost for McpHost {
    fn initial_read(&self) -> Option<(String, Value)> {
        Some(("inspect_document".into(), json!({})))
    }
    fn tools(&mut self, _target: &Target, cancel: &Cancellation) -> Result<Vec<ToolDefinition>, AiError> {
        cancel.check()?;
        let result = self.request("tools/list", json!({}))?;
        Ok(result.get("tools").and_then(Value::as_array).into_iter().flatten().filter_map(bound_tool).collect())
    }
    fn call(&mut self, _target: &Target, call: &ToolCall, cancel: &Cancellation) -> Result<Value, AiError> {
        cancel.check()?;
        if !TOOLS.contains(&call.name.as_str()) {
            return Err(AiError::invalid("this MCP tool is not available to the assistant"));
        }
        if call.name == "new_document" {
            builtin_template_params(&call.arguments).map_err(AiError::invalid)?;
        }
        if matches!(call.name.as_str(), "list_documents" | "ui_inspect" | "parity")
            && !call.arguments.as_object().is_some_and(serde_json::Map::is_empty)
        {
            return Err(AiError::invalid("this metadata tool takes no arguments or document IDs"));
        }
        let result = self.request("tools/call", json!({"name":call.name,"arguments":call.arguments}));
        cancel.check()?;
        result
    }
}

fn builtin_template_params(params: &Value) -> Result<(), &'static str> {
    if params.as_object().is_some_and(|params| {
        params.keys().all(|key| key == "template")
            && params.get("template").is_none_or(|template| matches!(template.as_str(), Some("blank" | "sample" | "letter" | "resume" | "report")))
    }) {
        Ok(())
    } else {
        Err("new_document requires only a built-in blank, sample, letter, resume or report template")
    }
}

/// Document commands and explicit visual setters are available; other app settings and global actions stay out.
fn allowed_command(id: &str) -> bool {
    if matches!(
        id,
        "review.addToDictionary"
            | "review.ignoreAll"
            | "review.proofing"
            | "review.trackingOptions"
            | "review.readAloud"
            | "table.draw"
            | "table.eraser"
            | "design.setDefault"
            | "insert.quickParts"
            | "insert.autoText"
    ) {
        return false;
    }
    const FAMILIES: &[&str] = &[
        "text.",
        "select.",
        "format.",
        "para.",
        "table.",
        "insert.",
        "layout.",
        "page.",
        "style.",
        "styles.",
        "review.",
        "document.",
        "shape.",
        "equation.",
        "header.",
        "footer.",
        "list.",
        "note.",
        "references.",
        "design.",
    ];
    FAMILIES.iter().any(|family| id.starts_with(family))
        || matches!(
            id,
            "file.open"
                | "file.new"
                | "file.save"
                | "file.saveAs"
                | "file.exportPdf"
                | "file.exportPng"
                | "edit.undo"
                | "edit.redo"
                | "edit.replace"
                | "edit.replaceAll"
                | "view.darkMode"
                | "view.readPageColor"
                | "view.interfaceTheme"
                | "view.switchWindows"
        )
}

fn visual_command(id: &str) -> bool {
    matches!(id, "view.darkMode" | "view.readPageColor" | "view.interfaceTheme")
}

fn interface_theme(app: &WordApp) -> Value {
    json!({"setting":app.ui.theme.code(), "effective":if app.ui_is_dark() {"dark"} else {"light"}})
}

fn list_request(method: &str, params: &Value) -> bool {
    method == "engine.execute"
        && params.get("command").and_then(Value::as_str) == Some("view.switchWindows")
        && params.get("params").is_none_or(|params| params.as_object().is_some_and(serde_json::Map::is_empty))
}

fn read_request(method: &str, params: &Value) -> bool {
    method == "document.inspect"
        || (method == "engine.execute"
            && matches!(params.get("command").and_then(Value::as_str), Some("document.text" | "document.inspect" | "document.paragraph")))
}

pub(super) fn dispatch(app: &mut WordApp, ctx: &egui::Context, guard: &mut Guard, pending: Pending) {
    let result = admitted(app, ctx, guard, &pending);
    let _ = pending.reply.send(result);
}

fn admitted(app: &mut WordApp, ctx: &egui::Context, guard: &mut Guard, pending: &Pending) -> Result<Value, String> {
    pending.cancel.check().map_err(|e| e.to_string())?;
    if Instant::now() >= pending.expires {
        return Err("assistant request expired before execution".into());
    }
    if app.session.document_id() != guard.document || app.windows.shown() != guard.window || (guard.had_document && !app.has_document()) {
        guard.read = false;
        guard.visual_read = false;
        return Err("the target document changed or closed; start a new request in the intended document".into());
    }
    if pending.params.get("doc").is_some_and(|doc| doc.as_u64() != Some(guard.document)) {
        return Err("this turn is bound to the original document".into());
    }
    if SelectionState::new(&app.session) != guard.selection {
        guard.read = false;
        guard.visual_read = false;
        pending.cancel.cancel();
        return Err("the selection changed; start a new request for the intended selection".into());
    }
    let changed = app.session.rev() != guard.revision;
    let read = read_request(&pending.method, &pending.params);
    let list = list_request(&pending.method, &pending.params);
    if changed {
        guard.read = false;
        guard.visual_read = false;
        if !read && !list && !matches!(pending.method.as_str(), "engine.commands" | "ui.inspect" | "ui.parity") {
            if pending.params.get("command").and_then(Value::as_str).is_some_and(visual_command) {
                return Err("document changed; ui_inspect again before visual changes".into());
            }
            return Err("document changed; inspect_document again before editing".into());
        }
    }
    if pending.method == "engine.commands" {
        let mut commands = app.session.registry.describe();
        if let Some(commands) = commands.as_array_mut() {
            commands.retain(|command| command.get("id").and_then(Value::as_str).is_some_and(allowed_command));
            for command in commands {
                let params = match command.get("id").and_then(Value::as_str) {
                    Some("view.darkMode") => Some(r#"{"value": bool} (explicit setter; call ui_inspect first)"#),
                    Some("view.readPageColor") => Some(r#"{"value": "none" | "sepia" | "inverse"} (explicit setter; call ui_inspect first)"#),
                    Some("view.interfaceTheme") => Some(r#"{"value": "light" | "dark" | "system"} (explicit setter; call ui_inspect first)"#),
                    Some("view.switchWindows") => Some(r#"{} → open-document metadata only; no switching or content access"#),
                    _ => None,
                };
                if let Some(params) = params
                    && let Some(command) = command.as_object_mut()
                {
                    command.insert("params".into(), json!(params));
                }
            }
        }
        return Ok(commands);
    }
    if pending.method == "ui.inspect" {
        guard.visual_read = true;
        guard.revision = app.session.rev();
        return Ok(json!({
            "document": guard.document,
            "view": {"darkMode":app.session.view.dark_mode, "readPageColor":app.session.view.read_page_color},
            "interfaceTheme": interface_theme(app),
        }));
    }
    if !matches!(pending.method.as_str(), "engine.execute" | "document.inspect" | "ui.parity") {
        return Err("this control method is unavailable to the assistant".into());
    }
    if pending.method == "engine.execute" {
        let command = pending.params.get("command").and_then(Value::as_str).ok_or("missing command")?;
        if !allowed_command(command) {
            return Err("this command changes app state outside the assistant's document scope".into());
        }
        if command == "view.switchWindows" && !list {
            return Err("list_documents cannot change the bound document".into());
        }
        if command == "file.new"
            && let Some(params) = pending.params.get("params")
        {
            builtin_template_params(params).map_err(String::from)?;
        }
        let visual = visual_command(command);
        if visual {
            if !guard.read && !guard.visual_read {
                return Err("ui_inspect before changing the page display or interface theme".into());
            }
            let value = pending.params.pointer("/params/value");
            if if command == "view.darkMode" { value.and_then(Value::as_bool).is_none() } else { value.and_then(Value::as_str).is_none() } {
                return Err("visual changes require an explicit value; inspect the current state rather than blindly toggling".into());
            }
        }
        if (command == "review.language"
            && (pending.params.pointer("/params/detect").is_some()
                || pending.params.pointer("/params/default").and_then(Value::as_bool) == Some(true)))
            || (command == "review.wordCount" && pending.params.pointer("/params/includeTextBoxes").is_some())
        {
            return Err("these parameters change preferences outside the assistant's document scope".into());
        }
        if matches!(command, "edit.replace" | "edit.replaceAll") {
            let replacement = pending.params.pointer("/params/with").and_then(Value::as_str).unwrap_or(&app.session.find.replace);
            let special = pending.params.pointer("/params/special").and_then(Value::as_bool).unwrap_or(!app.session.find.literal);
            if special && replacement.contains("^c") {
                return Err("clipboard replacements are outside the assistant's document scope; provide replacement text or special:false".into());
            }
        }
        if !guard.read && !(visual && guard.visual_read) && !read && !list && !matches!(command, "file.open" | "file.new") {
            return Err("inspect_document before changing the document".into());
        }
    }
    let grants = guard.grants.as_ref().map_err(Clone::clone)?;
    let (request, _) = control::ControlRequest::new(&pending.method, pending.params.clone());
    // An already-open file skips engine I/O; it still requires the same file grant.
    if pending.params.get("command").and_then(Value::as_str) == Some("file.open") {
        let path = pending.params.pointer("/params/path").and_then(Value::as_str).ok_or("file.open requires an explicitly authorized path")?;
        confine_files(grants, || wordcraft_engine::file_access::check_read(path))?;
    }
    let theme_setter = pending.method == "engine.execute"
        && pending.params.get("command").and_then(Value::as_str) == Some("view.interfaceTheme")
        && matches!(pending.params.pointer("/params/value").and_then(Value::as_str), Some("light" | "dark" | "system"));
    if theme_setter && !app.session.ui_requests.is_empty() {
        return Err("wait for pending UI requests before changing the interface theme".into());
    }
    app.assistant.requested_dialog = false;
    // This explicit setter emits one non-dialog UI request; all other frontend requests stay blocked.
    app.assistant.dispatching = !theme_setter;
    let response = confine_files(grants, || control::handle(app, ctx, &request));
    app.assistant.dispatching = false;
    let mut result = match response {
        control::Outcome::Done(value) if value.get("ok").and_then(Value::as_bool) == Some(true) => {
            Ok(value.get("result").cloned().unwrap_or(Value::Null))
        }
        control::Outcome::Done(value) => Err(value.get("error").and_then(Value::as_str).unwrap_or("command failed").to_string()),
        control::Outcome::Screenshot { .. } => Err("screenshots are unavailable in this assistant".into()),
    };
    if app.assistant.requested_dialog {
        result = Err("this command needs explicit parameters; assistant commands cannot open dialogs".into());
    }
    if app.session.document_id() != guard.document {
        if result.is_ok() && matches!(pending.params.get("command").and_then(Value::as_str), Some("file.open" | "file.new")) {
            // Only our successful authorized open or built-in creation may retarget a turn.
            guard.document = app.session.document_id();
            guard.window = app.windows.shown();
            guard.had_document = app.has_document();
            guard.read = false;
            guard.visual_read = false;
        } else {
            pending.cancel.cancel();
        }
    }
    guard.revision = app.session.rev();
    guard.selection = SelectionState::new(&app.session);
    if read && result.is_ok() {
        guard.read = true;
    }
    if (pending.method == "document.inspect" || pending.params.get("command").and_then(Value::as_str) == Some("document.inspect"))
        && let Ok(Value::Object(body)) = &mut result
    {
        body.insert("interfaceTheme".into(), interface_theme(app));
        guard.visual_read = true;
    }
    result
}

#[cfg(test)]
mod tests;
