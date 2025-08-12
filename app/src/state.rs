use crate::state::content_editor::ContentEditorState;
use crate::views::ViewID;
use egui::Context;
use egui_notify::Toasts;
use serde::{Deserialize, Serialize};

mod content_editor;

#[derive(Default, Serialize, Deserialize)]
pub struct AppState {
    current_view: ViewID,
    #[serde(skip)]
    toasts: Toasts,
    pub content_editor_state: ContentEditorState,
}

impl AppState {
    pub fn render(&mut self, ctx: &Context) {
        self.toasts.show(ctx);
    }

    pub fn current_view(&self) -> ViewID {
        self.current_view
    }

    pub fn switch_view(&mut self, view: ViewID) {
        self.current_view = view;
    }

    pub fn toast_error(&mut self, message: &str) {
        self.toasts.error(message).duration(None);
    }

    pub fn toast_success(&mut self, message: &str) {
        self.toasts.success(message).duration(None);
    }
}
