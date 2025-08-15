use crate::components::data_pack_meta_edit::DataPackMetaEdit;
use crate::components::Component;
use crate::state::AppState;
use crate::systems::file_picker::{FilePickerConfig, FilePickerMode};
use crate::windows::ViewWindow;
use egui::{Id, Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DataPackWindow {
    open: bool,
}

impl DataPackWindow {
    fn on_restore_core_data_clicked(&self, state: &mut AppState) {
        match state.content_editor.restore_core_data() {
            Ok(_) => state.toasts.success("Core data restored"),
            Err(e) => {
                state.toasts.error(e.to_string());
            }
        }
    }

    fn on_dump_json_clicked(&self, state: &mut AppState) {
        state.file_picker.open(
            FilePickerConfig::save()
                .mode(FilePickerMode::Save)
                .default_file_name("datapack.json"),
            |state, path| match state.content_editor.edited_pack.write_json_file(&path) {
                Ok(_) => state.toasts.success(format!("Data dumped to {:?}", path)),
                Err(e) => state.toasts.error(format!("Failed to dump data: {}", e)),
            },
        );
    }

    fn on_export_idct_clicked(&self, state: &mut AppState) {
        state.file_picker.open(
            FilePickerConfig::save()
                .mode(FilePickerMode::Save)
                .default_file_name("datapack.idct"),
            |state, path| match state.content_editor.edited_pack.write_idct_file(&path) {
                Ok(_) => state
                    .toasts
                    .success(format!("Data pack exported to {:?}", path)),
                Err(e) => state
                    .toasts
                    .error(format!("Failed to export data pack: {}", e)),
            },
        )
    }
}

impl ViewWindow for DataPackWindow {
    fn id(&self) -> Id {
        Id::new("content_editor_data_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Data Pack"
    }

    fn is_open(&self) -> bool {
        self.open
    }

    fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    fn render_content(&mut self, ui: &mut Ui, state: &mut AppState) {
        DataPackMetaEdit::new(&mut state.content_editor.edited_pack.meta).show(ui);

        ui.separator();

        if ui.button("Restore Core").clicked() {
            self.on_restore_core_data_clicked(state);
        }

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Dump as JSON").clicked() {
                self.on_dump_json_clicked(state);
            }

            if ui.button("Export as IDCT Datapack").clicked() {
                self.on_export_idct_clicked(state);
            }
        });
    }
}
