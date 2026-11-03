//! Software updates, background check at startup, and startup prompt modal.

use crate::WordApp;

#[cfg(not(target_arch = "wasm32"))]
use craft_update::dialog::{CheckResultDialog, CheckResultResponse};
#[cfg(not(target_arch = "wasm32"))]
use craft_update::updater::{Action, Status, Updater};

pub(crate) struct Runtime {
    disabled: bool,
    custom_source: bool,
    #[cfg(not(target_arch = "wasm32"))]
    updater: Updater,
    #[cfg(not(target_arch = "wasm32"))]
    result_dialog: CheckResultDialog,
    /// The desktop app turns the check at start on; tests and the web build leave it off.
    #[cfg(not(target_arch = "wasm32"))]
    startup_check: bool,
    #[cfg(not(target_arch = "wasm32"))]
    check_requested: bool,
    /// The context of the last frame, to wake the UI when a check is queued from a menu callback.
    #[cfg(not(target_arch = "wasm32"))]
    repaint: Option<egui::Context>,
    pub(crate) manual_window_open: bool,
    pub(crate) status_message: Option<String>,
}

impl Default for Runtime {
    fn default() -> Self {
        Self {
            disabled: false,
            custom_source: false,
            #[cfg(not(target_arch = "wasm32"))]
            updater: Updater::new(&crate::UPDATE_APP, env!("CARGO_PKG_VERSION")),
            #[cfg(not(target_arch = "wasm32"))]
            result_dialog: CheckResultDialog::new(&crate::UPDATE_APP, env!("CARGO_PKG_VERSION")),
            #[cfg(not(target_arch = "wasm32"))]
            startup_check: false,
            #[cfg(not(target_arch = "wasm32"))]
            check_requested: false,
            #[cfg(not(target_arch = "wasm32"))]
            repaint: None,
            manual_window_open: false,
            status_message: None,
        }
    }
}

impl WordApp {
    /// Uses only explicitly configured assistant-enabled releases; never falls back upstream.
    /// Call when attaching the independently installed assistant bridge. An explicit update
    /// opt-out remains in force.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn configure_assistant_updates(&mut self, bridge_protocol: u32) {
        self.updates_rt.custom_source = true;
        let root = craft_ai_native::storage::SharedStore::default_root().map_err(|error| error.to_string());
        self.updates_rt.updater = craft_update::custom::updater(&crate::UPDATE_APP, env!("CARGO_PKG_VERSION"), root, bridge_protocol);
    }

    /// Separately packaged builds must not install a standard WordCraft release over themselves.
    pub fn disable_updates(&mut self) {
        self.updates_rt.disabled = true;
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.updates_rt.startup_check = false;
            self.updates_rt.check_requested = false;
        }
    }

    /// Whether the app asks for a newer release when it starts (off until the desktop app turns it on).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_startup_update_check(&mut self, on: bool) {
        self.updates_rt.startup_check = on;
    }
}

pub(crate) fn check(app: &mut WordApp) {
    if app.updates_rt.disabled {
        app.updates_rt.manual_window_open = true;
        app.updates_rt.status_message =
            Some(tl!("Software updates are disabled for this launch. Install an updated app package to update this build.").to_string());
        return;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        app.updates_rt.check_requested = true;
        app.updates_rt.manual_window_open = true;
        if let Some(ctx) = &app.updates_rt.repaint {
            ctx.request_repaint();
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        app.updates_rt.manual_window_open = true;
        app.updates_rt.status_message = Some(tl!("Update check is not supported in the web browser.").to_string());
    }
}

pub(crate) fn tick(app: &mut WordApp, ctx: &egui::Context) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let rt = &mut app.updates_rt;
        rt.repaint = Some(ctx.clone());
        if rt.disabled {
            return;
        }
        if std::mem::take(&mut rt.check_requested) {
            rt.updater.check_now(ctx);
        } else if rt.startup_check {
            rt.updater.check_at_start(ctx);
        }
        rt.updater.tick(ctx);
    }
    #[cfg(target_arch = "wasm32")]
    let _ = (app, ctx);
}

pub(crate) fn show(app: &mut WordApp, ctx: &egui::Context) {
    #[cfg(not(target_arch = "wasm32"))]
    if !app.updates_rt.disabled {
        // The restart helper is scheduled before the app closes, so no update starts over unsaved work.
        let clean = !(app.session.dirty || app.save_unconfirmed() || app.printing.busy() || crate::panes::has_comment_drafts(app));
        match app.updates_rt.updater.show_if(ctx, clean) {
            Action::OpenUrl(url) => ctx.open_url(egui::OpenUrl::new_tab(url)),
            Action::Close => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
            Action::None => {}
        }
        // The dialog answers Help ▸ Check for Updates; a found update leaves the status idle and the prompt above takes over.
        let rt = &mut app.updates_rt;
        if rt.manual_window_open && rt.custom_source {
            (rt.manual_window_open, rt.status_message) = match rt.updater.status() {
                Status::Idle => (false, None),
                Status::Checking => (true, Some(tl!("Checking for updates…").to_string())),
                Status::UpToDate => (
                    true,
                    Some(tl!("WordCraft {version} is currently the newest version available.").replace("{version}", env!("CARGO_PKG_VERSION"))),
                ),
                Status::Failed(error) => (true, Some(tl!("Could not check for updates:\n{error}").replace("{error}", error))),
            };
        } else if rt.manual_window_open {
            match rt.result_dialog.show(ctx, rt.updater.status()) {
                CheckResultResponse::None => {}
                CheckResultResponse::Close => {
                    rt.manual_window_open = false;
                    rt.updater.dismiss_status();
                }
                CheckResultResponse::CheckAgain => rt.updater.check_now(ctx),
            }
            // An update was found: the prompt replaces the dialog.
            if matches!(rt.updater.status(), Status::Idle) {
                rt.manual_window_open = false;
            }
        }
    }

    if app.updates_rt.manual_window_open && (app.updates_rt.disabled || app.updates_rt.custom_source || cfg!(target_arch = "wasm32")) {
        let mut open = true;
        let mut close = false;
        egui::Window::new(tl!("Check for Updates"))
            .open(&mut open)
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(repo) = app.updates_rt.updater.repository() {
                    ui.label(tl!("Update source: {repo}").replace("{repo}", repo));
                }
                if let Some(msg) = &app.updates_rt.status_message {
                    ui.label(msg);
                }
                if ui.button(tl!("Close")).clicked() {
                    close = true;
                }
            });
        if !open || close {
            app.updates_rt.manual_window_open = false;
            app.updates_rt.status_message = None;
            #[cfg(not(target_arch = "wasm32"))]
            app.updates_rt.updater.dismiss_status();
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    #[test]
    fn custom_source_failure_never_advertises_upstream_release_url() {
        let mut app = WordApp::new(wordcraft_engine::Session::new(wordcraft_doc::Document::new()), Default::default());
        app.updates_rt.custom_source = true;
        let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let no_pins = std::env::temp_dir().join(format!("wordcraft-v015-no-pins-{}-{nonce}", std::process::id()));
        app.updates_rt.updater = Updater::new(&crate::UPDATE_APP, env!("CARGO_PKG_VERSION"))
            .with_custom_policy(Err("synthetic custom source unavailable".into()))
            .with_pins_path(no_pins);
        let ctx = egui::Context::default();
        check(&mut app);
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
        while !matches!(app.updates_rt.updater.status(), Status::Failed(_)) {
            tick(&mut app, &ctx);
            assert!(std::time::Instant::now() < deadline, "local custom-source failure must finish without network");
            std::thread::yield_now();
        }
        // An egui window's first frame measures its area before painting it.
        for _ in 0..3 {
            ctx.run_ui(egui::RawInput::default(), |ui| show(&mut app, ui.ctx())).drop_without_applying_deltas();
        }
        let output = ctx.run_ui(egui::RawInput::default(), |ui| show(&mut app, ui.ctx()));
        let text = output
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) => Some(text.galley.text()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n");
        assert!(text.contains("synthetic custom source unavailable"), "{text}");
        assert!(!text.contains("storytold/wordcraft"), "{text}");
        assert!(!text.contains("All releases are listed"), "{text}");
        assert!(app.updates_rt.manual_window_open);
        output.drop_without_applying_deltas();
    }

    #[test]
    fn packaged_build_opt_out_blocks_startup_and_manual_updater_work() {
        let mut app = WordApp::new(wordcraft_engine::Session::new(wordcraft_doc::Document::new()), Default::default());
        assert!(!app.updates_rt.disabled, "standard builds retain their updater");
        app.set_startup_update_check(true);
        check(&mut app);
        assert!(app.updates_rt.check_requested);
        app.disable_updates();
        app.configure_assistant_updates(1);
        assert!(!app.updates_rt.startup_check);
        assert!(!app.updates_rt.check_requested);
        let ctx = egui::Context::default();
        for command in ["ui.checkForUpdates", "help.checkForUpdates"] {
            app.run(command, serde_json::json!({})).unwrap();
            ctx.run_ui(egui::RawInput::default(), |ui| {
                tick(&mut app, ui.ctx());
                show(&mut app, ui.ctx());
            })
            .drop_without_applying_deltas();
            assert!(!app.updates_rt.check_requested);
            assert!(matches!(app.updates_rt.updater.status(), Status::Idle));
            assert!(app.updates_rt.manual_window_open);
            assert!(app.updates_rt.status_message.as_deref().is_some_and(|s| s.contains("disabled")));
        }
    }
}
