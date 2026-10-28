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

const TOOLS: &[&str] =
    &["list_commands", "execute", "batch", "open_document", "save_document", "type_text", "select_text", "get_text", "inspect_document"];

impl ToolHost for McpHost {
    fn tools(&mut self, _target: &Target, cancel: &Cancellation) -> Result<Vec<ToolDefinition>, AiError> {
        cancel.check()?;
        let result = self.request("tools/list", json!({}))?;
        Ok(result
            .get("tools")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|tool| {
                let name = tool.get("name")?.as_str()?;
                TOOLS.contains(&name).then(|| ToolDefinition {
                    name: name.into(),
                    description: tool.get("description").and_then(Value::as_str).unwrap_or_default().into(),
                    input_schema: tool.get("inputSchema").cloned().unwrap_or_else(|| json!({"type":"object"})),
                })
            })
            .collect())
    }
    fn call(&mut self, _target: &Target, call: &ToolCall, cancel: &Cancellation) -> Result<Value, AiError> {
        cancel.check()?;
        if !TOOLS.contains(&call.name.as_str()) {
            return Err(AiError::invalid("this MCP tool is not available to the assistant"));
        }
        let result = self.request("tools/call", json!({"name":call.name,"arguments":call.arguments}));
        cancel.check()?;
        result
    }
}

/// Only document commands are available; app settings, macros, dialogs, and global actions stay out.
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
                | "file.save"
                | "file.saveAs"
                | "file.exportPdf"
                | "file.exportPng"
                | "edit.undo"
                | "edit.redo"
                | "edit.replace"
                | "edit.replaceAll"
        )
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
        return Err("the target document changed or closed; start a new request in the intended document".into());
    }
    if pending.params.get("doc").is_some_and(|doc| doc.as_u64() != Some(guard.document)) {
        return Err("this turn is bound to the original document".into());
    }
    if SelectionState::new(&app.session) != guard.selection {
        pending.cancel.cancel();
        return Err("the selection changed; start a new request for the intended selection".into());
    }
    let changed = app.session.rev() != guard.revision;
    let read = read_request(&pending.method, &pending.params);
    if changed && !read && pending.method != "engine.commands" {
        guard.read = false;
        return Err("document changed; inspect_document again before editing".into());
    }
    if pending.method == "engine.commands" {
        let mut commands = app.session.registry.describe();
        if let Some(commands) = commands.as_array_mut() {
            commands.retain(|command| command.get("id").and_then(Value::as_str).is_some_and(allowed_command));
        }
        return Ok(commands);
    }
    if !matches!(pending.method.as_str(), "engine.execute" | "document.inspect") {
        return Err("this control method is unavailable to the assistant".into());
    }
    if pending.method == "engine.execute" {
        let command = pending.params.get("command").and_then(Value::as_str).ok_or("missing command")?;
        if !allowed_command(command) {
            return Err("this command changes app state outside the assistant's document scope".into());
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
        if !guard.read && !read && command != "file.open" {
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
    app.assistant.requested_dialog = false;
    app.assistant.dispatching = true;
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
        if result.is_ok() && pending.params.get("command").and_then(Value::as_str) == Some("file.open") {
            // Only our successful, file-authorized open may change the target during a turn.
            guard.document = app.session.document_id();
            guard.window = app.windows.shown();
            guard.had_document = app.has_document();
            guard.read = false;
        } else {
            pending.cancel.cancel();
        }
    }
    guard.revision = app.session.rev();
    guard.selection = SelectionState::new(&app.session);
    if read && result.is_ok() {
        guard.read = true;
    }
    result
}

#[cfg(test)]
mod tests;
