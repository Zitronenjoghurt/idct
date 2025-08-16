use crate::state::content_editor::ContentEditorState;
use crate::systems::file_picker::FilePicker;
use crate::systems::toasts::ToastSystem;
use crate::views::ViewID;
use egui::Context;
use serde::{Deserialize, Serialize};

mod content_editor;

#[derive(Default, Serialize, Deserialize)]
pub struct AppState {
    current_view: ViewID,
    #[serde(skip)]
    pub toasts: ToastSystem,
    #[serde(skip)]
    pub file_picker: FilePicker,
    pub content_editor: ContentEditorState,
}

impl AppState {
    pub fn update(&mut self, ctx: &Context) {
        self.update_file_picker(ctx);
        self.toasts.update(ctx);
        self.content_editor.update();
    }

    pub fn current_view(&self) -> ViewID {
        self.current_view
    }

    pub fn switch_view(&mut self, view: ViewID) {
        self.current_view = view;
    }

    fn update_file_picker(&mut self, ctx: &Context) {
        let mut file_picker = std::mem::take(&mut self.file_picker);
        file_picker.update(ctx, self);
        self.file_picker = file_picker;
    }
}
