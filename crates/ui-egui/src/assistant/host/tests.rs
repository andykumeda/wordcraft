use super::*;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use wordcraft_engine::file_access::{self, AutomationRoots};

fn app() -> WordApp {
    WordApp::new(wordcraft_engine::Session::new(wordcraft_doc::Document::from_text("Original text")), crate::Services::default())
}
fn text(app: &WordApp) -> String {
    app.session.doc.plain_text(wordcraft_doc::StoryRef::Body)
}
fn target() -> Target {
    Target { id: "turn-correlation-only".into(), revision: 0 }
}

fn init(server: &mut Server) {
    server.handle(json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":wordcraft_mcp::server::PROTOCOL_VERSION,"capabilities":{},"clientInfo":{"name":"external-test-client","version":"1"}}}));
    server.handle(json!({"jsonrpc":"2.0","method":"notifications/initialized"}));
}

#[test]
fn side_panel_catalog_matches_all_external_mcp_names_descriptions_and_schemas() {
    let cancel = Cancellation::new();
    let (sender, _) = mpsc::channel();
    let mut host = McpHost::new(sender, cancel.clone(), egui::Context::default(), Duration::from_secs(10));
    let mut external = Server::new(Box::new(wordcraft_mcp::Headless::default()));
    init(&mut external);
    let raw = external.handle(json!({"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}})).unwrap()["result"].clone();
    assert_eq!(host.request("tools/list", json!({})).unwrap(), raw);
    let catalog = host.tools(&target(), &cancel).unwrap();
    let tools = raw["tools"].as_array().unwrap();
    assert_eq!(catalog.len(), tools.len());
    assert_eq!(catalog.len(), 17);
    for (tool, external) in catalog.iter().zip(tools) {
        assert_eq!(tool.name, external["name"].as_str().unwrap());
        assert_eq!(tool.description, external["description"].as_str().unwrap());
        assert_eq!(tool.input_schema, external["inputSchema"]);
        assert_eq!(tool.to_mcp(), *external, "all MCP descriptor fields must survive the client adapter");
    }
    for name in ["render_page", "screenshot", "click", "key"] {
        assert!(catalog.iter().any(|tool| tool.name == name));
    }
    assert!(host.initial_read().is_none(), "MCP operations have no assistant-only read prerequisite");
}

/// Exercise each MCP client against the same live-backend queue, without TCP or provider access.
fn mcp(app: &mut WordApp, calls: Vec<(&'static str, Value)>, side_panel: bool, roots: Option<Arc<AutomationRoots>>) -> Vec<Value> {
    let (sender, receiver) = mpsc::channel();
    let ctx = egui::Context::default();
    let wake = ctx.clone();
    let mut guard = Guard::new(app, String::new(), vec![]);
    let rect = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(8.0, 8.0));
    let mut raw = egui::RawInput { screen_rect: Some(rect), max_texture_side: Some(4096), ..Default::default() };
    raw.viewports.insert(
        egui::ViewportId::ROOT,
        egui::ViewportInfo {
            inner_rect: Some(rect),
            native_pixels_per_point: Some(1.0),
            minimized: Some(false),
            occluded: Some(false),
            ..Default::default()
        },
    );
    app.raw_input_hook(&ctx, &mut raw);
    app.refresh_capture_geometry(&ctx);
    let worker = std::thread::spawn(move || {
        let cancel = Cancellation::new();
        if side_panel {
            let mut host = McpHost::new(sender, cancel.clone(), wake, Duration::from_secs(10));
            calls
                .into_iter()
                .enumerate()
                .map(|(id, (name, arguments))| host.call(&target(), &ToolCall { id: id.to_string(), name: name.into(), arguments }, &cancel).unwrap())
                .collect()
        } else {
            let mut server = Server::new(Box::new(Channel { sender, cancel, wake, timeout: Duration::from_secs(10) }));
            init(&mut server);
            calls.into_iter().enumerate().map(|(id,(name,arguments))| server.handle(json!({"jsonrpc":"2.0","id":id+2,"method":"tools/call","params":{"name":name,"arguments":arguments}})).unwrap()["result"].clone()).collect()
        }
    });
    while let Ok(request) = receiver.recv_timeout(Duration::from_secs(15)) {
        file_access::confine(roots.as_ref(), || dispatch(app, &ctx, &mut guard, request));
        // Synthetic GPU completion goes through the production screenshot admission/encoding path.
        for shot in &mut app.shots.requests {
            shot.issued = true;
            if let Some(callback) = shot.callback.take() {
                callback.complete(Arc::new(egui::ColorImage::filled(shot.expected, egui::Color32::WHITE)));
            }
        }
        control::drain_screenshots(app, &ctx);
    }
    worker.join().unwrap()
}

#[test]
fn omitted_render_screenshot_click_and_key_calls_match_the_external_live_server() {
    let calls = vec![
        ("render_page", json!({"page":1,"scale":0.1})),
        ("screenshot", json!({})),
        ("click", json!({"x":2,"y":3})),
        ("key", json!({"key":"B","cmd":true})),
    ];
    let mut embedded = app();
    let mut external = app();
    let side = mcp(&mut embedded, calls.clone(), true, None);
    let outside = mcp(&mut external, calls, false, None);
    assert_eq!(side, outside);
    assert_eq!(side.len(), 4);
    for image in side.iter().take(2) {
        assert_eq!(image["isError"], false, "{image}");
        assert_eq!(image["content"][0]["type"], "image");
        assert_eq!(image["content"][0]["mimeType"], "image/png");
        assert!(image["content"][0]["data"].as_str().unwrap().starts_with("iVBOR"));
    }
    assert_eq!(embedded.synthetic, external.synthetic);
}

#[test]
fn command_discovery_and_non_document_commands_are_the_external_catalog() {
    let calls = vec![("list_commands", json!({})), ("execute", json!({"command":"view.interfaceTheme","params":{"value":"dark"}}))];
    let mut embedded = app();
    let mut external = app();
    let side = mcp(&mut embedded, calls.clone(), true, None);
    let outside = mcp(&mut external, calls, false, None);
    assert_eq!(side, outside);
    let commands: Value = serde_json::from_str(side[0]["content"][0]["text"].as_str().unwrap()).unwrap();
    for id in ["file.options", "file.close", "view.switchWindows", "ui.assistant.updateStatus"] {
        assert!(commands.as_array().unwrap().iter().any(|command| command["id"] == id), "{id}");
    }
    assert_eq!(side[1]["isError"], false);
}

#[test]
fn explicit_open_document_ids_and_edits_need_no_side_panel_read_or_file_grant() {
    let mut app = app();
    let first = app.session.document_id();
    app.execute("file.new", json!({"template":"blank"})).unwrap();
    let active = app.session.document_id();
    let results = mcp(
        &mut app,
        vec![
            ("type_text", json!({"text":" prefix","doc":first})),
            ("get_text", json!({"doc":first})),
            ("execute", json!({"command":"view.switchWindows","params":{"id":first}})),
        ],
        true,
        None,
    );
    assert!(results.iter().all(|result| result["isError"] == false));
    assert!(results[1]["content"][0]["text"].as_str().unwrap().contains("prefix"));
    assert_ne!(first, active);
    assert_eq!(app.session.document_id(), first);
}

#[test]
fn protected_documents_dirty_close_and_undo_keep_backend_validation() {
    let mut app = app();
    let original = text(&app);
    let results = mcp(
        &mut app,
        vec![
            ("type_text", json!({"text":"changed"})),
            ("execute", json!({"command":"edit.undo"})),
            ("execute", json!({"command":"text.insert","params":{"text":"unsaved"}})),
            ("execute", json!({"command":"file.close"})),
        ],
        true,
        None,
    );
    assert!(results.iter().take(3).all(|result| result["isError"] == false));
    assert_eq!(results[3]["isError"], true);
    assert!(results[3].to_string().contains("unsaved changes"));
    assert!(app.has_document());
    app.session.run("edit.undo", &json!({})).unwrap();
    assert_eq!(text(&app), original);
    app.session.run("review.restrict", &json!({"mode":"readOnly"})).unwrap();
    let denied = mcp(&mut app, vec![("type_text", json!({"text":"blocked"}))], true, None);
    assert_eq!(denied[0]["isError"], true);
    assert_eq!(text(&app), original);
}

struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!("wordcraft-mcp-parity-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn authoritative_file_roots_are_preserved_without_exact_file_grant_restrictions() {
    let temp = Temp::new();
    let allowed = temp.0.join("allowed");
    let outside = temp.0.join("outside");
    std::fs::create_dir(&allowed).unwrap();
    std::fs::create_dir(&outside).unwrap();
    let roots = AutomationRoots::new(Some(&allowed), Some(&allowed)).unwrap();
    let permitted = allowed.join("new.txt");
    let refused = outside.join("no.txt");
    let mut app = app();
    let results = mcp(&mut app, vec![("save_document", json!({"path":permitted})), ("save_document", json!({"path":refused}))], true, roots);
    assert_eq!(results[0]["isError"], false);
    assert_eq!(results[1]["isError"], true);
    assert!(permitted.exists());
    assert!(!refused.exists());
}

#[test]
fn unknown_and_malformed_calls_preserve_mcp_errors() {
    let calls = vec![("not_a_tool", json!({})), ("execute", json!({})), ("key", json!({"key":"not-a-key"}))];
    assert_eq!(mcp(&mut app(), calls.clone(), true, None), mcp(&mut app(), calls, false, None));
}

#[test]
fn cancelled_and_expired_queued_commands_never_execute() {
    for stopped in [true, false] {
        let mut app = app();
        let original = text(&app);
        let mut guard = Guard::new(&app, String::new(), vec![]);
        let (request, reply) = control::ControlRequest::new("engine.execute", json!({"command":"text.insert","params":{"text":"must not run"}}));
        let cancel = Cancellation::new();
        if stopped {
            cancel.cancel();
        }
        let expires = if stopped { Instant::now() + Duration::from_secs(5) } else { Instant::now() - Duration::from_secs(1) };
        dispatch(&mut app, &egui::Context::default(), &mut guard, Pending { request, cancel, expires });
        assert_eq!(reply.recv().unwrap()["ok"], false);
        assert_eq!(text(&app), original);
    }
}

#[test]
fn lost_acknowledgments_and_timeouts_are_cancelled_without_replay() {
    for timeout in [false, true] {
        let (sender, receiver) = mpsc::channel();
        let cancel = Cancellation::new();
        let worker_cancel = cancel.clone();
        let worker = std::thread::spawn(move || {
            let mut host = McpHost::new(
                sender,
                worker_cancel.clone(),
                egui::Context::default(),
                if timeout { Duration::from_millis(5) } else { Duration::from_secs(5) },
            );
            host.call(&target(), &ToolCall { id: "read".into(), name: "get_text".into(), arguments: json!({}) }, &worker_cancel)
        });
        let pending = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        if timeout {
            assert_eq!(worker.join().unwrap().unwrap_err().kind, ErrorKind::Cancelled);
            let mut app = app();
            let mut guard = Guard::new(&app, String::new(), vec![]);
            dispatch(&mut app, &egui::Context::default(), &mut guard, pending);
        } else {
            drop(pending);
            assert_eq!(worker.join().unwrap().unwrap_err().kind, ErrorKind::Cancelled);
        }
        assert!(cancel.is_cancelled());
    }
}
#[test]
fn assistant_pane_and_options_work_without_native_services_or_profile_access() {
    let mut app = app();
    assert!(!app.assistant.enabled);
    app.execute("ui.assistant", json!({"value":true})).unwrap();
    assert!(crate::panes::open(&app).contains(&crate::panes::Pane::Assistant));
    let commands = crate::assistant::commands();
    assert!(commands.iter().any(|entry| entry.get("id").and_then(Value::as_str) == Some("ui.assistant")));
    let status = app.execute("ui.assistant.status", json!({})).unwrap();
    assert_eq!(status.get("available"), Some(&Value::Bool(false)));
    app.execute("ui.backstage", json!({"value":true,"page":"options","pane":"agents"})).unwrap();
    assert_eq!(app.ui.options_pane, "agents");
    assert!(app.ui.backstage);
    app.execute("ui.assistant", json!({"value":false})).unwrap();
    assert!(!crate::panes::open(&app).contains(&crate::panes::Pane::Assistant));
}

#[test]
#[ignore = "manual headless visual capture of synthetic WordCraft content"]
fn capture_wordcraft_assistant_pane() {
    let mut app = app();
    super::super::native::enable_without_profile(&mut app);
    app.execute("ui.assistant", json!({"value":true})).unwrap();
    app.assistant.widget.chat_route = "Synthetic local connection / chat-model".into();
    app.assistant.widget.control_route = "Synthetic hosted connection / control-model".into();
    app.assistant.widget.activity.push(craft_ui::assistant::Activity {
        label: "format.bold".into(),
        detail: "Synthetic tool result: complete; Undo available.".into(),
        failed: false,
    });
    app.assistant.widget.messages = vec![
        craft_ai_core::assistant::Message::text(craft_ai_core::assistant::MessageRole::User, "Make the title bold."),
        craft_ai_core::assistant::Message::text(craft_ai_core::assistant::MessageRole::Assistant, "The requested edit is ready to review."),
    ];
    let mut harness = egui_kittest::Harness::builder().with_size(egui::vec2(1440.0, 900.0)).build_ui_state(
        |ui, app: &mut WordApp| {
            let ctx = ui.ctx().clone();
            app.logic(&ctx);
            app.ui(ui);
        },
        app,
    );
    harness.run_steps(5);
    let path = std::env::temp_dir().join("wordcraft-assistant-pane.png");
    harness.render().unwrap().save(&path).unwrap();
    eprintln!("{}", path.display());
}
