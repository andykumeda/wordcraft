//! WordCraft's small host adapter for the reusable Craft assistant.

use craft_ui::assistant::{Action, State};
use serde_json::{Value, json};

use crate::WordApp;

#[cfg(not(target_arch = "wasm32"))]
mod host;
#[cfg(not(target_arch = "wasm32"))]
mod native;

#[derive(Default)]
pub struct Assistant {
    pub open: bool,
    pub widget: State,
    pub enabled: bool,
    pub(crate) dispatching: bool,
    pub(crate) requested_dialog: bool,
    #[cfg(not(target_arch = "wasm32"))]
    native: native::Native,
}

impl Drop for Assistant {
    fn drop(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        self.native.cancel();
    }
}

pub fn commands() -> Vec<Value> {
    vec![
        json!({"id":"ui.assistant","label":"Assistant","location":"View › Assistant","params":"value?: bool","enabled":true}),
        json!({"id":"ui.assistant.send","label":"Send assistant prompt","location":"Agents","params":"text: string; files?: array of exact paths","enabled":cfg!(not(target_arch="wasm32"))}),
        json!({"id":"ui.assistant.newChat","label":"New assistant chat","location":"Agents","params":"{}","enabled":cfg!(not(target_arch="wasm32"))}),
        json!({"id":"ui.assistant.stop","label":"Stop assistant","location":"Agents","params":"{}","enabled":true}),
        json!({"id":"ui.assistant.status","label":"Assistant status","location":"Agents","params":"{}","enabled":true}),
    ]
}

pub(crate) fn command(app: &mut WordApp, id: &str, params: &Value) -> Option<Result<Value, String>> {
    if !id.starts_with("ui.assistant") {
        return None;
    }
    let result = match id {
        "ui.assistant" => {
            app.assistant.open = params.get("value").and_then(Value::as_bool).unwrap_or(!app.assistant.open);
            ensure_loaded(app);
            Ok(json!({"open":app.assistant.open}))
        }
        "ui.assistant.send" => {
            let Some(text) = params.get("text").and_then(Value::as_str).filter(|s| !s.trim().is_empty()) else {
                return Some(Err("give a nonempty text prompt".into()));
            };
            if let Some(files) = params.get("files") {
                let Some(files) = files.as_array().filter(|files| files.len() <= 32) else {
                    return Some(Err("files must contain at most 32 exact file paths".into()));
                };
                let Some(files) = files.iter().map(|v| v.as_str().map(str::to_owned)).collect::<Option<Vec<_>>>() else {
                    return Some(Err("file paths must be strings".into()));
                };
                app.assistant.widget.authorized_files = files;
            }
            app.assistant.open = true;
            action(app, Action::Send(text.into())).map(|()| json!({"started":true}))
        }
        "ui.assistant.newChat" => action(app, Action::NewChat).map(|()| json!({"started":true})),
        "ui.assistant.stop" => action(app, Action::Stop).map(|()| json!({"stopped":true})),
        "ui.assistant.status" => Ok(
            json!({"open":app.assistant.open,"busy":app.assistant.widget.busy,"available":app.assistant.enabled,"settingsBusy":app.assistant.widget.settings.busy,"chatId":app.assistant.widget.active_chat,"model":app.assistant.widget.chat_route,"chatModel":app.assistant.widget.chat_route,"controlModel":app.assistant.widget.chat_route,"status":app.assistant.widget.status,"error":app.assistant.widget.error}),
        ),
        _ => Err("unknown assistant command".into()),
    };
    Some(result)
}

pub fn pane(app: &mut WordApp, ui: &mut egui::Ui) {
    ensure_loaded(app);
    if !app.assistant.widget.busy {
        app.assistant.widget.target_label = app.title_stem();
    }
    let id = app.window_id("assistant");
    let actions = craft_ui::assistant::show(ui, id, &mut app.assistant.widget, |text| tl!(text).into());
    apply_actions(app, actions);
}

pub fn settings(app: &mut WordApp, ui: &mut egui::Ui) {
    ensure_loaded(app);
    if !app.assistant.enabled {
        ui.label(tl!("The assistant is available in the desktop app."));
        return;
    }
    let id = app.window_id("assistant_settings");
    let actions = craft_ui::assistant::settings(ui, id, &mut app.assistant.widget.settings, |text| tl!(text).into());
    apply_actions(app, actions);
}

fn apply_actions(app: &mut WordApp, actions: Vec<Action>) {
    for requested in actions {
        if let Err(error) = action(app, requested) {
            app.assistant.widget.error = Some(error.clone());
            app.assistant.widget.settings.error = Some(error);
        }
    }
}

fn ensure_loaded(app: &mut WordApp) {
    #[cfg(not(target_arch = "wasm32"))]
    native::ensure_loaded(app);
    #[cfg(target_arch = "wasm32")]
    let _ = app;
}

fn action(app: &mut WordApp, action: Action) -> Result<(), String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        native::action(app, action)
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = (app, action);
        Err("The assistant is available in the desktop app.".into())
    }
}

pub(crate) fn poll(app: &mut WordApp, ctx: &egui::Context) {
    #[cfg(not(target_arch = "wasm32"))]
    native::poll(app, ctx);
    #[cfg(target_arch = "wasm32")]
    let _ = (app, ctx);
}
