//! The side panel is an MCP client of the same live GUI backend used by external clients.

use std::sync::mpsc;
use std::time::{Duration, Instant};

use craft_ai_core::assistant::{Target, ToolCall, ToolDefinition};
use craft_ai_core::{AiError, ErrorKind};
use craft_ai_native::{Cancellation, ToolHost};
use serde_json::{Value, json};
use wordcraft_mcp::{backend::Backend, server::Server};

use crate::{WordApp, control};

pub(super) struct Pending {
    request: control::ControlRequest,
    cancel: Cancellation,
    expires: Instant,
}

/// The originating document and window identify implicit requests for this turn.
/// The live backend still validates explicit documents, files and dirty-close requests.
pub(super) struct Guard {
    target: Target,
    window: u64,
}

impl Guard {
    pub(super) fn target(&self) -> Target {
        self.target.clone()
    }

    pub(super) fn new(app: &WordApp, _prompt: String, _files: Vec<String>) -> Self {
        Self { target: Target { id: app.session.document_id().to_string(), revision: app.session.rev() }, window: app.windows.shown() }
    }
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
        let (request, receiver) = control::ControlRequest::new(method, params);
        let expires = Instant::now().checked_add(self.timeout).ok_or("assistant timeout is too large")?;
        self.sender
            .send(Pending { request, cancel: self.cancel.clone(), expires })
            .map_err(|_| "the app stopped answering; the command outcome is unknown and will not be replayed")?;
        self.wake.request_repaint();
        loop {
            self.cancel.check().map_err(|e| e.to_string())?;
            match receiver.recv_timeout(Duration::from_millis(50)) {
                Ok(value) => {
                    return if value.get("ok").and_then(Value::as_bool) == Some(true) {
                        Ok(value.get("result").cloned().unwrap_or(Value::Null))
                    } else {
                        Err(value.get("error").and_then(Value::as_str).unwrap_or("control request failed").to_owned())
                    };
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    self.cancel.cancel();
                    return Err("the app stopped answering; the command outcome is unknown and will not be replayed".into());
                }
                Err(mpsc::RecvTimeoutError::Timeout) if Instant::now() >= expires => {
                    self.cancel.cancel();
                    return Err("the app did not answer; the command outcome is unknown and will not be replayed".into());
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {}
            }
        }
    }
    fn has_ui(&self) -> bool {
        true
    }
    fn describe(&self) -> String {
        "connected to the WordCraft app through private assistant pipes".into()
    }
}

pub(super) struct McpHost {
    server: Server,
}

impl McpHost {
    pub(super) fn new(sender: mpsc::Sender<Pending>, cancel: Cancellation, wake: egui::Context, timeout: Duration) -> Self {
        let mut server = Server::new(Box::new(Channel { sender, cancel, wake, timeout }));
        server.handle(json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":wordcraft_mcp::server::PROTOCOL_VERSION,"capabilities":{},"clientInfo":{"name":"craft-assistant","version":"1"}}}));
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

impl ToolHost for McpHost {
    fn tools(&mut self, _target: &Target, cancel: &Cancellation) -> Result<Vec<ToolDefinition>, AiError> {
        cancel.check()?;
        let result = self.request("tools/list", json!({}))?;
        let tools = result.get("tools").and_then(Value::as_array).ok_or_else(|| AiError::invalid("MCP returned an invalid tool catalog"))?;
        tools.iter().map(ToolDefinition::from_mcp).collect()
    }
    fn call(&mut self, _target: &Target, call: &ToolCall, cancel: &Cancellation) -> Result<Value, AiError> {
        cancel.check()?;
        let result = self.request("tools/call", json!({"name":call.name,"arguments":call.arguments}));
        cancel.check()?;
        result
    }
}

pub(super) fn dispatch(app: &mut WordApp, ctx: &egui::Context, guard: &mut Guard, pending: Pending) {
    if pending.cancel.is_cancelled() || Instant::now() >= pending.expires {
        pending.cancel.cancel();
        let _ = pending.request.reply.send(json!({"ok":false,"error":"assistant request expired or was stopped before execution"}));
        return;
    }
    let request = &pending.request;
    if request.method.starts_with("ui.") && !matches!(request.method.as_str(), "ui.inspect" | "ui.parity" | "ui.render" | "ui.screenshot") {
        let _ = pending.request.reply.send(json!({"ok":false,"error":"Assistant visual input is unavailable because the target window can change before the action completes. Use document commands instead."}));
        return;
    }
    let addressed = matches!(request.method.as_str(), "engine.execute" | "command" | "document.inspect" | "ui.render")
        && request.params.get("doc").is_some_and(|doc| !doc.is_null());
    let discovery = matches!(request.method.as_str(), "engine.commands" | "ui.inspect" | "ui.parity")
        || (matches!(request.method.as_str(), "engine.execute" | "command")
            && request.params.get("command").or_else(|| request.params.get("id")).and_then(Value::as_str) == Some("view.switchWindows"));
    if guard.window != app.windows.shown()
        && !discovery
        && (!addressed || request.params.get("doc").and_then(Value::as_u64) == Some(app.session.document_id()))
    {
        let _ = pending.request.reply.send(json!({"ok":false,"error":"The active window changed during this assistant turn. Start a new turn in the intended window, or explicitly address another document with `doc`."}));
        return;
    }
    if guard.target.id != app.session.document_id().to_string() && !addressed && !discovery {
        let _ = pending.request.reply.send(json!({"ok":false,"error":"The active document changed during this assistant turn. Specify the document id with `doc`, or start a new turn in the intended window."}));
        return;
    }
    // Document commands keep the control channel's validation and file-root checks.
    match control::handle(app, ctx, &pending.request) {
        control::Outcome::Done(value) => {
            let _ = pending.request.reply.send(value);
        }
        control::Outcome::Screenshot { path } => control::admit_screenshot(app, ctx, path, pending.request.reply),
    }
}

#[cfg(test)]
mod tests;
