use crate::state::AppState;
use crate::windows::ViewWindow;
use egui::{Id, Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SettingsWindow {
    open: bool,
}

impl ViewWindow for SettingsWindow {
    fn id(&self) -> Id {
        Id::new("settings_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Settings"
    }

    fn is_open(&self) -> bool {
        self.open
    }

    fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    fn render_content(&mut self, _ui: &mut Ui, _state: &mut AppState) {
        //let mut pixels_per_point = ui.ctx().pixels_per_point();
        //ui.add(Slider::new(&mut pixels_per_point, 0.1..=2.0));
        //ui.ctx().set_pixels_per_point(pixels_per_point);
    }
}
