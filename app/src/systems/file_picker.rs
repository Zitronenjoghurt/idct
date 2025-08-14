use crate::state::AppState;
use egui::Context;
use egui_file_dialog::{FileDialog, FileDialogConfig};
use std::path::PathBuf;

pub struct FilePicker {
    file_dialog: FileDialog,
    callback: Option<Box<dyn FnOnce(&mut AppState, PathBuf)>>,
}

impl Default for FilePicker {
    fn default() -> Self {
        Self {
            file_dialog: FileDialog::default().add_save_extension("JSON", "json"),
            callback: None,
        }
    }
}

impl FilePicker {
    pub fn open<F>(&mut self, config: FilePickerConfig, callback: F)
    where
        F: FnOnce(&mut AppState, PathBuf) + 'static,
    {
        *self.file_dialog.config_mut() = config.dialog_config;
        self.callback = Some(Box::new(callback));
        match config.mode {
            FilePickerMode::Pick => self.file_dialog.pick_file(),
            FilePickerMode::Save => self.file_dialog.save_file(),
        }
    }

    pub fn update(&mut self, ctx: &Context, state: &mut AppState) {
        self.file_dialog.update(ctx);

        if let Some(path) = self.file_dialog.take_picked()
            && let Some(callback) = self.callback.take()
        {
            callback(state, path);
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FilePickerMode {
    #[default]
    Pick,
    Save,
}

#[derive(Default)]
pub struct FilePickerConfig {
    mode: FilePickerMode,
    dialog_config: FileDialogConfig,
}

impl FilePickerConfig {
    pub fn pick() -> Self {
        Self {
            mode: FilePickerMode::Pick,
            ..Default::default()
        }
    }

    pub fn save() -> Self {
        Self {
            mode: FilePickerMode::Save,
            ..Default::default()
        }
    }

    pub fn mode(mut self, mode: FilePickerMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn default_file_name(mut self, default_file_name: &str) -> Self {
        self.dialog_config.default_file_name = default_file_name.to_string();
        self
    }
}
