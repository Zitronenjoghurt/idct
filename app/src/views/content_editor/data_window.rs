use crate::state::AppState;
use crate::systems::file_picker::{FilePickerConfig, FilePickerMode};
use crate::windows::ViewWindow;
use egui::{Id, Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DataWindow {
    open: bool,
}

impl ViewWindow for DataWindow {
    fn id(&self) -> Id {
        Id::new("content_editor_data_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Data Handling"
    }

    fn is_open(&self) -> bool {
        self.open
    }

    fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    fn render_content(&mut self, ui: &mut Ui, state: &mut AppState) {
        if ui.button("Restore Core Data").clicked() {
            match state.content_editor_state.restore_core_data() {
                Ok(_) => state.toasts.success("Core data restored"),
                Err(e) => {
                    state.toasts.error(e.to_string());
                }
            }
        }

        ui.separator();

        if ui.button("Dump Data").clicked() {
            state.file_picker.open(
                FilePickerConfig::save()
                    .mode(FilePickerMode::Save)
                    .default_file_name("data_dump.json"),
                |state, path| {
                    let data = &state.content_editor_state.edited_data;
                    match std::fs::write(&path, serde_json::to_string_pretty(data).unwrap()) {
                        Ok(_) => state.toasts.success(format!("Data dumped to {:?}", path)),
                        Err(e) => state.toasts.error(format!("Failed to dump data: {}", e)),
                    }
                },
            );
        }
    }
}
