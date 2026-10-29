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
        assert!(tools.iter().any(|tool| tool.name == "new_document"));
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
    let listing = tools.iter().find(|tool| tool.name == "list_documents").unwrap();
    assert_eq!(listing.input_schema["properties"], json!({}));
    assert!(listing.description.contains("metadata") && listing.description.contains("without changing the active document"));
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
    for id in ["file.options", "macro.run", "ui.assistant.send", "file.close"] {
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

#[test]
fn initial_context_uses_the_bound_document_inspector_without_editing() {
    let (sender, _receiver) = mpsc::channel();
    let host = McpHost::new(sender, Cancellation::new(), egui::Context::default(), Duration::from_secs(10));
    let (name, arguments) = host.initial_read().unwrap();
    assert_eq!(name, "inspect_document");
    let mut app = app();
    let mut guard = Guard::new(&app, "remove the dark background".into(), vec![]);
    let revision = app.session.rev();
    let results = mcp(&mut app, &mut guard, vec![("inspect_document", arguments)]);
    assert!(results[0].to_string().contains("Original text"));
    assert!(guard.read);
    assert_eq!(app.session.rev(), revision);
    assert_eq!(text(&app), "Original text");
}

#[test]
fn visual_inspection_and_explicit_setters_do_not_read_or_edit_document_content() {
    use crate::theme::Appearance;
    let mut app = app();
    app.execute("design.pageColor", json!({"color":"202020"})).unwrap();
    app.execute("view.darkMode", json!({"value":true})).unwrap();
    app.execute("view.readPageColor", json!({"value":"inverse"})).unwrap();
    app.ui.theme = Appearance::Dark;
    let original = app.session.doc.clone();
    let revision = app.session.rev();
    let dirty = app.session.dirty;
    let undo = app.session.can_undo();
    let mut expected_prefs = serde_json::to_value(app.prefs()).unwrap();
    expected_prefs["theme"] = json!("light");
    // The explicit dark-page setter is also reflected in the normal visual preferences.
    expected_prefs["darkPage"] = json!(false);
    let mut guard = Guard::new(&app, "Remove the screen's dark background".into(), vec![]);
    let inspected = mcp(&mut app, &mut guard, vec![("ui_inspect", json!({}))]);
    let body: Value = serde_json::from_str(inspected[0]["content"][0]["text"].as_str().unwrap()).unwrap();
    assert_eq!(body.as_object().unwrap().len(), 3);
    assert_eq!(body["document"], guard.document);
    assert_eq!(body["interfaceTheme"], json!({"setting":"dark","effective":"dark"}));
    assert_eq!(body["view"], json!({"darkMode":true,"readPageColor":"inverse"}));
    assert!(guard.visual_read);
    assert!(!guard.read, "visual metadata is not document-content authorization");
    let results = mcp(
        &mut app,
        &mut guard,
        vec![
            ("execute", json!({"command":"view.darkMode","params":{"value":false}})),
            ("execute", json!({"command":"view.readPageColor","params":{"value":"none"}})),
            ("execute", json!({"command":"view.interfaceTheme","params":{"value":"light"}})),
        ],
    );
    assert!(results.iter().all(|result| result["isError"] == false), "{results:?}");
    assert!(!app.session.view.dark_mode);
    assert_eq!(app.session.view.read_page_color, wordcraft_engine::cmd::view::ReadPageColor::None);
    assert_eq!(app.ui.theme, Appearance::Light);
    assert_eq!(serde_json::to_value(app.prefs()).unwrap(), expected_prefs);
    assert_eq!(app.session.doc, original);
    assert_eq!(app.session.rev(), revision);
    assert_eq!(app.session.dirty, dirty);
    assert_eq!(app.session.can_undo(), undo);
    assert!(app.dialog.is_none() && app.file_dialog.is_none());
    assert!(!app.assistant.requested_dialog && !app.assistant.dispatching);
    assert!(
        command(&mut app, &mut guard, "text.insert", json!({"text":"not authorized by visual inspection"})).unwrap_err().contains("inspect_document")
    );
    for id in ["view.darkMode", "view.readPageColor", "view.interfaceTheme"] {
        assert!(command(&mut app, &mut guard, id, json!({})).unwrap_err().contains("explicit value"));
    }
    let inspection = mcp(&mut app, &mut guard, vec![("inspect_document", json!({}))]);
    let body: Value = serde_json::from_str(inspection[0]["content"][0]["text"].as_str().unwrap()).unwrap();
    assert_eq!(body["interfaceTheme"], json!({"setting":"light","effective":"light"}));
    assert_eq!(body["pageBackground"]["color"], "202020");
    assert!(guard.read && guard.visual_read);
}

#[test]
fn system_theme_inspection_reports_the_effective_os_appearance() {
    let mut app = app();
    let ctx = egui::Context::default();
    app.ctx = Some(ctx.clone());
    app.ui.theme = crate::theme::Appearance::System;
    let mut guard = Guard::new(&app, "Inspect the interface appearance".into(), vec![]);
    for (system, effective) in [(egui::Theme::Dark, "dark"), (egui::Theme::Light, "light")] {
        let output = ctx.run_ui(egui::RawInput { system_theme: Some(system), ..Default::default() }, |_| {});
        output.drop_without_applying_deltas();
        let body = admitted(&mut app, &ctx, &mut guard, &pending("ui.inspect", json!({}))).unwrap();
        assert_eq!(body["interfaceTheme"], json!({"setting":"system","effective":effective}));
        let external = control::inspect(&mut app, &ctx);
        assert_eq!(body["interfaceTheme"], external["interfaceTheme"]);
        assert_eq!(body["view"]["darkMode"], external["view"]["darkMode"]);
        assert_eq!(body["view"]["readPageColor"], external["view"]["readPageColor"]);
        assert!(external["view"].get("zoom").is_some(), "existing external view fields remain");
    }
    assert!(!guard.read);
}

#[test]
fn visual_commands_still_refuse_stale_selection_and_foreign_document_context() {
    let mut app = app();
    app.session.run("view.darkMode", &json!({"value":true})).unwrap();
    let mut guard = Guard::new(&app, "Use light pages".into(), vec![]);
    admitted(&mut app, &egui::Context::default(), &mut guard, &pending("ui.inspect", json!({}))).unwrap();
    app.session.run("text.insert", &json!({"text":"user edit "})).unwrap();
    app.session.sel = guard.selection.text.clone();
    assert!(command(&mut app, &mut guard, "view.darkMode", json!({"value":false})).unwrap_err().contains("changed"));
    assert!(!guard.visual_read && app.session.view.dark_mode);
    admitted(&mut app, &egui::Context::default(), &mut guard, &pending("ui.inspect", json!({}))).unwrap();
    assert!(!guard.read && guard.visual_read);
    let wrong = pending("engine.execute", json!({"command":"view.darkMode","params":{"value":false},"doc":u64::MAX}));
    assert!(admitted(&mut app, &egui::Context::default(), &mut guard, &wrong).unwrap_err().contains("original document"));
    assert!(app.session.view.dark_mode);
    app.session.run("select.all", &json!({})).unwrap();
    assert!(command(&mut app, &mut guard, "view.darkMode", json!({"value":false})).unwrap_err().contains("selection changed"));
    assert!(!guard.visual_read);
    assert!(app.session.view.dark_mode);
    app.session = wordcraft_engine::Session::new(wordcraft_doc::Document::from_text("Other document"));
    assert!(command(&mut app, &mut guard, "view.interfaceTheme", json!({"value":"light"})).unwrap_err().contains("changed or closed"));
    assert_eq!(text(&app), "Other document");
}

#[test]
fn new_document_preserves_existing_writing_and_requires_fresh_content_inspection() {
    let mut app = app();
    app.session.run("text.insert", &json!({"text":"Unsaved "})).unwrap();
    let original_id = app.session.document_id();
    let original = app.session.doc.clone();
    assert!(app.session.dirty);
    let mut guard = Guard::new(&app, "Create a blank document".into(), vec![]);
    let created = mcp(&mut app, &mut guard, vec![("new_document", json!({"template":"blank"}))]);
    assert_eq!(created[0]["isError"], false, "{created:?}");
    assert_ne!(guard.document, original_id);
    assert_eq!(guard.document, app.session.document_id());
    assert!(!guard.read && !guard.visual_read);
    assert_eq!(app.session.with_document(original_id, |session| (session.doc.clone(), session.dirty)).unwrap(), (original.clone(), true));
    assert!(command(&mut app, &mut guard, "text.insert", json!({"text":"uninspected"})).unwrap_err().contains("inspect_document"));
    let results =
        mcp(&mut app, &mut guard, vec![("inspect_document", json!({})), ("type_text", json!({"text":"New writing"})), ("get_text", json!({}))]);
    assert!(results.iter().all(|result| result["isError"] == false), "{results:?}");
    assert_eq!(text(&app), "New writing");
    assert_eq!(app.session.with_document(original_id, |session| session.doc.clone()).unwrap(), original);
    let wrong = pending("engine.execute", json!({"command":"text.insert","params":{"text":"wrong document"},"doc":original_id}));
    assert!(admitted(&mut app, &egui::Context::default(), &mut guard, &wrong).unwrap_err().contains("original document"));
}

#[test]
fn document_listing_is_read_only_and_does_not_grant_content_or_switching_access() {
    let mut app = app();
    let original_id = app.session.document_id();
    let other_id = app.session.add_document(wordcraft_doc::Document::from_text("Other writing"));
    app.session.activate(original_id).unwrap();
    let mut guard = Guard::new(&app, "List the open documents".into(), vec![]);
    let listing = mcp(&mut app, &mut guard, vec![("list_documents", json!({}))]);
    assert_eq!(listing[0]["isError"], false, "{listing:?}");
    let body: Value = serde_json::from_str(listing[0]["content"][0]["text"].as_str().unwrap()).unwrap();
    assert_eq!(body["documents"].as_array().unwrap().len(), 2);
    assert_eq!(app.session.document_id(), original_id);
    assert!(!guard.read && !guard.visual_read);
    assert!(command(&mut app, &mut guard, "text.insert", json!({"text":"not authorized by listing"})).unwrap_err().contains("inspect_document"));
    for params in [json!({"id":other_id}), json!({"id":null}), json!({"index":0})] {
        assert!(command(&mut app, &mut guard, "view.switchWindows", params).unwrap_err().contains("cannot change the bound document"));
    }
    assert_eq!(app.session.document_id(), original_id);
}

#[test]
fn new_document_rejects_non_builtin_templates_before_the_mcp_adapter_can_default_them() {
    let cancel = Cancellation::new();
    let (sender, _receiver) = mpsc::channel();
    let mut host = McpHost::new(sender, cancel.clone(), egui::Context::default(), Duration::from_secs(10));
    let target = Target { id: "synthetic-target".into(), revision: 0 };
    for arguments in [
        json!({"template":"/tmp/private-template.docx"}),
        json!({"template":null}),
        json!({"template":42}),
        json!({"path":"/tmp/private-template.docx"}),
    ] {
        assert!(host.call(&target, &ToolCall { id: "invalid".into(), name: "new_document".into(), arguments }, &cancel).is_err());
    }
    let mut app = app();
    let original_id = app.session.document_id();
    let mut guard = Guard::new(&app, "Create a document".into(), vec![]);
    assert!(command(&mut app, &mut guard, "file.new", json!({"template":"/tmp/private-template.docx"})).unwrap_err().contains("built-in"));
    assert_eq!(app.session.document_id(), original_id);
}

#[test]
fn parity_is_read_only_metadata_and_metadata_tools_reject_arguments() {
    let mut app = app();
    let original = app.session.doc.clone();
    let revision = app.session.rev();
    let mut guard = Guard::new(&app, "What features are supported?".into(), vec![]);
    let results = mcp(&mut app, &mut guard, vec![("parity", json!({}))]);
    assert_eq!(results[0]["isError"], false);
    let body: Value = serde_json::from_str(results[0]["content"][0]["text"].as_str().unwrap()).unwrap();
    assert_eq!(body, wordcraft_engine::catalog::parity(&app.session.registry));
    assert!(!guard.read && !guard.visual_read);
    assert_eq!(app.session.rev(), revision);
    assert_eq!(app.session.doc, original);
    assert!(command(&mut app, &mut guard, "text.insert", json!({"text":"not authorized by parity"})).unwrap_err().contains("inspect_document"));
    let cancel = Cancellation::new();
    let (sender, _receiver) = mpsc::channel();
    let mut host = McpHost::new(sender, cancel.clone(), egui::Context::default(), Duration::from_secs(10));
    let target = Target { id: "synthetic-target".into(), revision: 0 };
    for name in ["parity", "list_documents", "ui_inspect"] {
        for arguments in [json!({"doc":u64::MAX}), json!({"other":true}), json!([])] {
            assert!(host.call(&target, &ToolCall { id: "invalid".into(), name: name.into(), arguments }, &cancel).is_err());
        }
    }
}

#[test]
fn bound_command_discovery_documents_only_the_supported_visual_and_listing_parameters() {
    let mut app = app();
    let mut guard = Guard::new(&app, "Inspect the app's visual controls".into(), vec![]);
    let commands = admitted(&mut app, &egui::Context::default(), &mut guard, &pending("engine.commands", json!({}))).unwrap();
    let commands = commands.as_array().unwrap();
    for id in ["view.darkMode", "view.readPageColor", "view.interfaceTheme"] {
        let entry = commands.iter().find(|command| command["id"] == id).unwrap();
        let params = entry["params"].as_str().unwrap();
        assert!(params.contains("explicit setter") && params.contains("ui_inspect"));
        assert!(!params.contains("?"));
        assert!(command(&mut app, &mut guard, id, json!({"value":false})).unwrap_err().contains("ui_inspect"));
    }
    let listed = commands.iter().find(|command| command["id"] == "view.switchWindows").unwrap();
    let params = listed["params"].as_str().unwrap();
    assert!(params.starts_with("{}") && params.contains("metadata only"));
    assert!(!params.contains("id") && !params.contains("index"));
    let external = app.session.registry.get("view.switchWindows").unwrap();
    assert!(external.params.contains(r#""id""#), "the external registry keeps full addressing");
    assert!(!guard.read && !guard.visual_read);
}

#[test]
fn interface_theme_exception_refuses_malformed_values_and_preserves_pending_ui_requests() {
    let mut app = app();
    app.execute("view.interfaceTheme", json!({"value":"dark"})).unwrap();
    let original = app.session.doc.clone();
    let prefs = serde_json::to_value(app.prefs()).unwrap();
    let revision = app.session.rev();
    let dirty = app.session.dirty;
    let undo = app.session.can_undo();
    let mut guard = Guard::new(&app, "Use a light interface".into(), vec![]);
    admitted(&mut app, &egui::Context::default(), &mut guard, &pending("ui.inspect", json!({}))).unwrap();
    for value in [json!(null), json!(false), json!(12), json!("sepia")] {
        assert!(command(&mut app, &mut guard, "view.interfaceTheme", json!({"value":value})).is_err());
        assert_eq!(serde_json::to_value(app.prefs()).unwrap(), prefs);
        assert_eq!(app.session.doc, original);
        assert_eq!(app.session.rev(), revision);
        assert_eq!(app.session.dirty, dirty);
        assert_eq!(app.session.can_undo(), undo);
        assert!(app.dialog.is_none() && app.file_dialog.is_none());
        assert!(!app.assistant.requested_dialog && !app.assistant.dispatching);
    }
    let queued = json!({"focus":"navSearch"});
    app.session.push_ui_request(queued.clone());
    assert!(command(&mut app, &mut guard, "view.interfaceTheme", json!({"value":"light"})).unwrap_err().contains("pending UI requests"));
    assert_eq!(app.session.ui_requests, vec![queued]);
    assert_eq!(serde_json::to_value(app.prefs()).unwrap(), prefs);
    assert!(app.dialog.is_none() && app.file_dialog.is_none());
    assert!(!app.assistant.requested_dialog && !app.assistant.dispatching);
}
