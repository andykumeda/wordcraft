use super::*;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

fn app() -> WordApp {
    WordApp::new(wordcraft_engine::Session::new(wordcraft_doc::Document::from_text("Original text")), crate::Services::default())
}

fn text(app: &WordApp) -> String {
    app.session.doc.plain_text(wordcraft_doc::StoryRef::Body)
}

fn pending(method: &str, params: Value) -> Pending {
    Pending {
        method: method.into(),
        params,
        reply: mpsc::channel().0,
        cancel: Cancellation::new(),
        expires: Instant::now() + Duration::from_secs(10),
    }
}

fn command(app: &mut WordApp, guard: &mut Guard, id: &str, params: Value) -> Result<Value, String> {
    admitted(app, &egui::Context::default(), guard, &pending("engine.execute", json!({"command":id,"params":params})))
}

fn read(app: &mut WordApp, guard: &mut Guard) {
    command(app, guard, "document.text", json!({})).unwrap();
}

/// Exercise the actual embedded MCP server on its worker and dispatch its queued calls on this
/// thread, just as the app does; no TCP listener, model, secret store or real document is involved.
fn mcp(app: &mut WordApp, guard: &mut Guard, calls: Vec<(&'static str, Value)>) -> Vec<Value> {
    let (sender, receiver) = mpsc::channel();
    let target = Target { id: guard.document.to_string(), revision: guard.revision };
    let ctx = egui::Context::default();
    let wake = ctx.clone();
    let worker = std::thread::spawn(move || {
        let cancel = Cancellation::new();
        let mut host = McpHost::new(sender, cancel.clone(), wake, Duration::from_secs(10));
        let tools = host.tools(&target, &cancel).unwrap();
        assert!(tools.iter().any(|tool| tool.name == "get_text"));
        assert!(!tools.iter().any(|tool| tool.name == "new_document"));
        calls
            .into_iter()
            .enumerate()
            .map(|(id, (name, arguments))| host.call(&target, &ToolCall { id: id.to_string(), name: name.into(), arguments }, &cancel).unwrap())
            .collect::<Vec<_>>()
    });
    for request in receiver {
        dispatch(app, &ctx, guard, request);
    }
    worker.join().unwrap()
}

#[test]
fn assistant_catalog_has_a_bound_target_without_changing_external_mcp() {
    let cancel = Cancellation::new();
    let (sender, _receiver) = mpsc::channel();
    let mut host = McpHost::new(sender, cancel.clone(), egui::Context::default(), Duration::from_secs(10));
    let external = host.request("tools/list", json!({})).unwrap();
    let tools = host.tools(&Target { id: "synthetic-target".into(), revision: 0 }, &cancel).unwrap();
    assert_eq!(tools.len(), TOOLS.len());
    for tool in &tools {
        assert!(TOOLS.contains(&tool.name.as_str()));
        assert!(tool.input_schema.pointer("/properties/doc").is_none(), "{}", tool.name);
        assert!(!tool.input_schema.get("required").unwrap().as_array().unwrap().iter().any(|name| name == "doc"));
        assert_eq!(tool.input_schema.get("additionalProperties"), Some(&Value::Bool(false)));
        assert!(tool.description.contains("document bound to the assistant turn"));
        assert!(!tool.description.contains("`doc`"));
        assert!(!tool.description.contains("list_documents"));
        assert!(!tool.description.contains("view.switchWindows"));
        assert!(!tool.input_schema.to_string().contains("list_documents"));
    }
    let external_tools = external.get("tools").unwrap().as_array().unwrap();
    for name in ["execute", "batch", "save_document", "type_text", "select_text", "get_text", "inspect_document"] {
        let external_tool = external_tools.iter().find(|tool| tool.get("name").and_then(Value::as_str) == Some(name)).unwrap();
        assert_eq!(external_tool.pointer("/inputSchema/properties/doc/minimum"), Some(&json!(0)), "{name}");
        assert!(external_tool.pointer("/inputSchema/properties/doc/description").unwrap().as_str().unwrap().contains("list_documents"));
    }
    assert!(external_tools.iter().any(|tool| tool.get("name").and_then(Value::as_str) == Some("list_documents")));
    assert!(!tools.iter().any(|tool| tool.name == "list_documents"));
    assert_eq!(host.request("tools/list", json!({})).unwrap(), external);
    let typing = tools.iter().find(|tool| tool.name == "type_text").unwrap();
    assert_eq!(typing.input_schema.pointer("/properties/text/type"), Some(&json!("string")));
    assert_eq!(typing.input_schema.pointer("/properties/paragraphs/type"), Some(&json!("boolean")));
    assert_eq!(typing.input_schema.get("required"), Some(&json!(["text"])));
}

#[test]
fn explicit_foreign_document_ids_are_rejected_through_embedded_mcp() {
    let mut app = app();
    let mut guard = Guard::new(&app, "Edit this document".into(), vec![]);
    assert_ne!(guard.document, 0);
    read(&mut app, &mut guard);
    let original = text(&app);
    let revision = app.session.rev();
    for wrong in [0, u64::MAX] {
        let results = mcp(
            &mut app,
            &mut guard,
            vec![
                ("get_text", json!({"doc":wrong})),
                ("inspect_document", json!({"doc":wrong})),
                ("select_text", json!({"text":"Original","doc":wrong})),
                ("type_text", json!({"text":"wrong document edit","doc":wrong})),
                ("execute", json!({"command":"text.insert","params":{"text":"wrong document edit"},"doc":wrong})),
                ("batch", json!({"commands":[{"command":"text.insert","params":{"text":"wrong document edit"}}],"doc":wrong})),
            ],
        );
        assert_eq!(results.len(), 6);
        assert!(results.iter().all(|result| result.get("isError") == Some(&Value::Bool(true))), "{results:?}");
        assert!(results.iter().all(|result| result.to_string().contains("this turn is bound to the original document")), "{results:?}");
        assert_eq!(text(&app), original);
        assert_eq!(app.session.rev(), revision);
        assert_eq!(SelectionState::new(&app.session).text, guard.selection.text);
    }
    let results = mcp(&mut app, &mut guard, vec![("get_text", json!({}))]);
    assert_eq!(results.first().unwrap().get("isError"), Some(&Value::Bool(false)));
    assert!(results.first().unwrap().to_string().contains("Original text"));
}

#[test]
fn mcp_read_edit_readback_and_normal_undo_share_the_session() {
    let mut app = app();
    let mut guard = Guard::new(&app, "Replace the text".into(), vec![]);
    let original = text(&app);
    let results = mcp(
        &mut app,
        &mut guard,
        vec![
            ("get_text", json!({})),
            ("select_text", json!({"text":"Original"})),
            ("type_text", json!({"text":"Assistant replacement"})),
            ("get_text", json!({})),
            ("execute", json!({"command":"edit.undo"})),
            ("get_text", json!({})),
        ],
    );
    assert_eq!(results.len(), 6);
    assert!(results.iter().all(|result| result.get("isError").and_then(Value::as_bool) != Some(true)), "{results:?}");
    assert!(results.get(3).unwrap().to_string().contains("Assistant replacement text"));
    assert_eq!(text(&app), original);
    assert!(guard.read);
}

#[test]
fn mcp_batch_reports_partial_completion_without_replaying_edits() {
    let mut app = app();
    let original = text(&app);
    let mut guard = Guard::new(&app, "Edit".into(), vec![]);
    let results = mcp(
        &mut app,
        &mut guard,
        vec![
            ("get_text", json!({})),
            (
                "batch",
                json!({"commands":[
                    {"command":"text.insert","params":{"text":"committed "}},
                    {"command":"file.options","params":{"author":"not allowed"}},
                    {"command":"text.insert","params":{"text":"never executed "}}
                ]}),
            ),
        ],
    );
    assert_eq!(results.get(1).and_then(|result| result.get("isError")), Some(&Value::Bool(true)));
    assert_eq!(text(&app).matches("committed ").count(), 1);
    assert!(!text(&app).contains("never executed"));
    command(&mut app, &mut guard, "edit.undo", json!({})).unwrap();
    assert_eq!(text(&app), original);
}

#[test]
fn mutation_requires_read_and_a_fresh_revision_and_selection() {
    let mut app = app();
    let mut guard = Guard::new(&app, "Edit the current text".into(), vec![]);
    assert!(command(&mut app, &mut guard, "text.insert", json!({"text":"wrong"})).unwrap_err().contains("inspect_document"));
    read(&mut app, &mut guard);
    app.session.run("text.insert", &json!({"text":"user edit "})).unwrap();
    app.session.sel = guard.selection.text.clone();
    let user_text = text(&app);
    assert!(command(&mut app, &mut guard, "text.insert", json!({"text":"stale"})).unwrap_err().contains("changed"));
    assert_eq!(text(&app), user_text);
    read(&mut app, &mut guard);
    app.session.run("select.all", &json!({})).unwrap();
    assert!(command(&mut app, &mut guard, "text.insert", json!({"text":"stale selection"})).unwrap_err().contains("changed"));
    assert_eq!(text(&app), user_text);
    assert!(command(&mut app, &mut guard, "document.text", json!({})).unwrap_err().contains("selection changed"));
    assert_eq!(text(&app), user_text);
}

#[test]
fn switched_closed_and_forged_document_targets_are_rejected() {
    let mut app = app();
    let mut guard = Guard::new(&app, "Edit".into(), vec![]);
    read(&mut app, &mut guard);
    let wrong = pending("engine.execute", json!({"command":"text.insert","params":{"text":"wrong"},"doc":u64::MAX}));
    assert!(admitted(&mut app, &egui::Context::default(), &mut guard, &wrong).unwrap_err().contains("original document"));
    app.session = wordcraft_engine::Session::new(wordcraft_doc::Document::from_text("Other document"));
    assert!(command(&mut app, &mut guard, "document.text", json!({})).unwrap_err().contains("changed or closed"));
    assert_eq!(text(&app).trim(), "Other document");
    let mut guard = Guard::new(&app, "Edit".into(), vec![]);
    app.start_screen();
    assert!(!app.has_document());
    assert!(command(&mut app, &mut guard, "document.text", json!({})).unwrap_err().contains("changed or closed"));
}

#[test]
fn cancelled_and_expired_queued_commands_never_execute() {
    let mut app = app();
    let mut guard = Guard::new(&app, "Edit".into(), vec![]);
    read(&mut app, &mut guard);
    for expired in [false, true] {
        let mut request = pending("engine.execute", json!({"command":"text.insert","params":{"text":"late edit"}}));
        if expired {
            request.expires = Instant::now() - Duration::from_secs(1);
        } else {
            request.cancel.cancel();
        }
        let original = text(&app);
        let revision = app.session.rev();
        let (reply, result) = mpsc::channel();
        request.reply = reply;
        dispatch(&mut app, &egui::Context::default(), &mut guard, request);
        assert!(result.recv().unwrap().is_err());
        assert_eq!(text(&app), original);
        assert_eq!(app.session.rev(), revision);
    }
}

#[test]
fn another_window_on_the_same_document_is_not_the_originating_window() {
    let mut app = app();
    app.set_window_builder(egui::ViewportBuilder::default());
    let mut guard = Guard::new(&app, "Edit the original window".into(), vec![]);
    read(&mut app, &mut guard);
    app.new_window();
    assert_ne!(app.windows.shown(), guard.window);
    assert_eq!(app.session.document_id(), guard.document);
    assert!(command(&mut app, &mut guard, "text.insert", json!({"text":"wrong window"})).is_err());
    assert_eq!(text(&app).trim(), "Original text");
}

#[test]
fn extra_object_selection_changes_invalidate_the_turn_without_a_text_caret_change() {
    let mut app = app();
    for kind in ["rectangle", "ellipse"] {
        app.session.run("insert.shape", &json!({"kind":kind})).unwrap();
        app.session.run("arrange.bounds", &json!({"page":0,"x":100,"y":100})).unwrap();
        if kind == "rectangle" {
            app.session.run("select.collapse", &json!({"end":true})).unwrap();
        }
    }
    let mut guard = Guard::new(&app, "Change this shape".into(), vec![]);
    read(&mut app, &mut guard);
    app.session.run("select.addObject", &json!({"index":0})).unwrap();
    assert_eq!(app.session.sel, guard.selection.text);
    assert_eq!(app.session.rev(), guard.revision);
    assert!(command(&mut app, &mut guard, "shape.fill", json!({"color":"FF0000"})).unwrap_err().contains("selection changed"));
    assert_eq!(app.session.rev(), guard.revision);
}

#[test]
fn column_and_equation_caret_changes_also_invalidate_the_turn() {
    for equation in [false, true] {
        let mut app = app();
        let mut guard = Guard::new(&app, "Edit selection".into(), vec![]);
        read(&mut app, &mut guard);
        if equation {
            app.session.math = Some(wordcraft_engine::MathEdit { at: app.session.sel.focus.clone(), pos: Default::default(), session: 42 });
        } else {
            app.session.column = Some(wordcraft_engine::ColumnBlock {
                sel: app.session.sel.clone(),
                anchor_x: 10.0,
                focus_x: 30.0,
                segments: Vec::new(),
                cells: None,
            });
        }
        assert_eq!(app.session.sel, guard.selection.text);
        assert!(command(&mut app, &mut guard, "text.insert", json!({"text":"wrong target"})).unwrap_err().contains("selection changed"));
    }
}

#[test]
fn protected_document_and_out_of_scope_commands_stay_protected() {
    let mut app = app();
    app.session.run("review.restrict", &json!({"mode":"readOnly"})).unwrap();
    let mut guard = Guard::new(&app, "Edit".into(), vec![]);
    read(&mut app, &mut guard);
    let original = text(&app);
    assert!(command(&mut app, &mut guard, "text.insert", json!({"text":"blocked"})).is_err());
    for id in ["file.options", "macro.run", "ui.assistant.send", "view.switchWindows", "file.close"] {
        assert!(command(&mut app, &mut guard, id, json!({})).is_err(), "{id}");
    }
    let commands = admitted(&mut app, &egui::Context::default(), &mut guard, &pending("engine.commands", json!({}))).unwrap();
    let ids: Vec<_> = commands.as_array().unwrap().iter().filter_map(|command| command.get("id").and_then(Value::as_str)).collect();
    assert!(ids.contains(&"format.bold"));
    assert!(!ids.contains(&"file.options"));
    assert_eq!(text(&app), original);
}

#[test]
fn document_tools_cannot_change_preferences_or_start_app_services() {
    let mut app = app();
    let mut guard = Guard::new(&app, "Edit this text".into(), vec![]);
    read(&mut app, &mut guard);
    for id in [
        "review.addToDictionary",
        "review.ignoreAll",
        "review.proofing",
        "review.trackingOptions",
        "review.readAloud",
        "table.draw",
        "table.eraser",
        "design.setDefault",
        "insert.quickParts",
        "insert.autoText",
    ] {
        assert!(!allowed_command(id), "{id}");
        assert!(command(&mut app, &mut guard, id, json!({})).is_err(), "{id}");
    }
    for (id, params) in [
        ("review.language", json!({"detect":false})),
        ("review.language", json!({"lang":"fr-FR","default":true})),
        ("review.wordCount", json!({"includeTextBoxes":false})),
    ] {
        assert!(command(&mut app, &mut guard, id, params).unwrap_err().contains("preferences"));
    }
    assert!(command(&mut app, &mut guard, "review.wordCount", json!({})).is_ok());
}

#[test]
fn replacements_cannot_read_shared_clipboard_content() {
    let mut app = app();
    app.session.clipboard_text = "other document private content".into();
    let mut guard = Guard::new(&app, "Replace Original".into(), vec![]);
    read(&mut app, &mut guard);
    assert!(command(&mut app, &mut guard, "edit.replaceAll", json!({"text":"Original","with":"^c"})).unwrap_err().contains("clipboard"));
    app.session.find.replace = "^c".into();
    assert!(command(&mut app, &mut guard, "edit.replaceAll", json!({"text":"Original"})).unwrap_err().contains("clipboard"));
    command(&mut app, &mut guard, "edit.replaceAll", json!({"text":"Original","with":"^c","special":false})).unwrap();
    assert_eq!(text(&app).trim(), "^c text");
}

#[test]
fn lost_acknowledgments_and_timeouts_are_cancelled_not_completed_tool_results() {
    for timeout in [false, true] {
        let (sender, receiver) = mpsc::channel();
        let cancel = Cancellation::new();
        let worker_cancel = cancel.clone();
        let worker = std::thread::spawn(move || {
            let duration = if timeout { Duration::from_millis(5) } else { Duration::from_secs(5) };
            let mut host = McpHost::new(sender, worker_cancel.clone(), egui::Context::default(), duration);
            host.call(
                &Target { id: "original".into(), revision: 0 },
                &ToolCall { id: "read".into(), name: "get_text".into(), arguments: json!({}) },
                &worker_cancel,
            )
        });
        let request = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        if timeout {
            assert_eq!(worker.join().unwrap().unwrap_err().kind, ErrorKind::Cancelled);
            let mut app = app();
            let mut guard = Guard::new(&app, "Read".into(), vec![]);
            assert!(admitted(&mut app, &egui::Context::default(), &mut guard, &request).is_err());
        } else {
            drop(request);
            assert_eq!(worker.join().unwrap().unwrap_err().kind, ErrorKind::Cancelled);
        }
        assert!(cancel.is_cancelled());
    }
}

struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!("wordcraft-assistant-host-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn path(&self, name: &str) -> String {
        self.0.join(name).to_string_lossy().into_owned()
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn exact_file_grants_allow_save_without_granting_neighbor_files() {
    let directory = Temp::new();
    let allowed = directory.path("allowed.txt");
    let forbidden = directory.path("neighbor.txt");
    let mut app = app();
    let mut guard = Guard::new(&app, "Save this document".into(), vec![allowed.clone()]);
    read(&mut app, &mut guard);
    assert!(command(&mut app, &mut guard, "file.save", json!({"path":forbidden})).is_err());
    assert!(!std::path::Path::new(&forbidden).exists());
    command(&mut app, &mut guard, "file.save", json!({"path":allowed})).unwrap();
    assert_eq!(std::fs::read_to_string(&allowed).unwrap().trim(), "Original text");
}

#[test]
fn a_named_file_prefix_does_not_authorize_a_different_file() {
    let directory = Temp::new();
    let requested = directory.path("report.txt");
    let mut app = app();
    let mut guard = Guard::new(&app, format!("Save to {requested}.backup"), vec![]);
    read(&mut app, &mut guard);
    assert!(command(&mut app, &mut guard, "file.save", json!({"path":requested})).is_err());
    assert!(!std::path::Path::new(&requested).exists());
}

#[test]
fn an_exact_quoted_output_path_with_spaces_is_authorized() {
    let directory = Temp::new();
    let requested = directory.path("new output.txt");
    let mut app = app();
    let mut guard = Guard::new(&app, format!("Save to \"{requested}\"."), vec![]);
    read(&mut app, &mut guard);
    command(&mut app, &mut guard, "file.save", json!({"path":requested})).unwrap();
    assert!(std::path::Path::new(&requested).is_file());
}

#[test]
fn named_paths_are_exact_and_unclosed_quotes_do_not_grant_files() {
    let a = std::env::temp_dir().join("a.txt").to_string_lossy().into_owned();
    let b = std::env::temp_dir().join("b space.txt").to_string_lossy().into_owned();
    assert_eq!(named_files(&format!("Read {a} and `{b}` please")), vec![a.clone(), b.clone()]);
    assert!(named_files(&format!("Read \"{b}")).is_empty());
    assert!(named_files(&format!("Read prefix{a}")).is_empty());
}

#[test]
fn already_open_files_still_need_a_file_grant() {
    let directory = Temp::new();
    let path = directory.path("other.txt");
    std::fs::write(&path, "Other document").unwrap();
    let mut app = app();
    let original = app.session.document_id();
    app.execute("file.open", json!({"path":path})).unwrap();
    app.session.activate(original).unwrap();
    let mut guard = Guard::new(&app, "Edit this document".into(), vec![]);
    assert!(command(&mut app, &mut guard, "file.open", json!({"path":path})).is_err());
    assert_eq!(app.session.document_id(), original);
}

#[test]
fn a_turn_started_without_a_document_can_open_an_authorized_file() {
    let directory = Temp::new();
    let path = directory.path("chosen.txt");
    std::fs::write(&path, "Chosen content").unwrap();
    let mut app = app();
    app.start_screen();
    assert!(!app.has_document());
    let mut guard = Guard::new(&app, "Open the chosen file".into(), vec![path.clone()]);
    command(&mut app, &mut guard, "file.open", json!({"path":path})).unwrap();
    assert!(app.has_document());
    assert_eq!(text(&app).trim(), "Chosen content");
    read(&mut app, &mut guard);
}

#[test]
fn commands_never_open_a_dialog_instead_of_answering() {
    let mut app = app();
    let mut guard = Guard::new(&app, "Insert a picture".into(), vec![]);
    read(&mut app, &mut guard);
    assert!(command(&mut app, &mut guard, "insert.picture", json!({})).unwrap_err().contains("explicit parameters"));
    assert!(app.dialog.is_none());
    assert!(app.file_dialog.is_none());
}

#[cfg(unix)]
#[test]
fn file_grants_are_captured_before_inference() {
    let directory = Temp::new();
    let chosen = directory.path("chosen.txt");
    let first = directory.path("first.txt");
    let other = directory.path("other.txt");
    std::fs::write(&first, "First").unwrap();
    std::fs::write(&other, "Other").unwrap();
    std::os::unix::fs::symlink(&first, &chosen).unwrap();
    let mut app = app();
    let mut guard = Guard::new(&app, format!("Open {chosen}"), vec![]);
    std::fs::remove_file(&chosen).unwrap();
    std::os::unix::fs::symlink(&other, &chosen).unwrap();
    assert!(command(&mut app, &mut guard, "file.open", json!({"path":chosen})).is_err());
    assert_eq!(text(&app).trim(), "Original text");
}

#[test]
fn opening_allowed_html_does_not_grant_its_linked_picture() {
    let directory = Temp::new();
    let page = directory.path("page.html");
    let picture = directory.path("picture.png");
    std::fs::write(&page, "<p>Example <img src=\"picture.png\" alt=\"Linked picture\"></p>").unwrap();
    image::RgbaImage::from_pixel(1, 1, image::Rgba([20, 40, 60, 255])).save(&picture).unwrap();
    for grant_picture in [false, true] {
        let mut app = app();
        let mut files = vec![page.clone()];
        if grant_picture {
            files.push(picture.clone());
        }
        let mut guard = Guard::new(&app, "Open the chosen file".into(), files);
        let request = pending("engine.execute", json!({"command":"file.open","params":{"path":page}}));
        admitted(&mut app, &egui::Context::default(), &mut guard, &request).unwrap();
        assert_eq!(app.session.doc.media.len(), usize::from(grant_picture));
        assert!(!request.cancel.is_cancelled(), "an explicitly authorized open reports success");
        assert_eq!(guard.document, app.session.document_id());
        assert!(command(&mut app, &mut guard, "text.insert", json!({"text":"unread"})).is_err());
        read(&mut app, &mut guard);
        command(&mut app, &mut guard, "text.insert", json!({"text":"read first "})).unwrap();
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
