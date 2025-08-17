use crate::components::dimension::dimension_definitions_edit::DimensionDefinitionsEdit;
use crate::components::Component;
use crate::state::AppState;
use crate::windows::ViewWindow;
use egui::{Id, Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DimensionWindow {
    open: bool,
}

impl ViewWindow for DimensionWindow {
    fn id(&self) -> Id {
        Id::new("content_editor_dimensions_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Dimensions"
    }

    fn is_open(&self) -> bool {
        self.open
    }

    fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    fn render_content(&mut self, ui: &mut Ui, state: &mut AppState) {
        DimensionDefinitionsEdit::new(&mut state.content_editor.edited_pack.data.dimensions)
            .show(ui);
    }

    fn resizable(&self) -> bool {
        false
    }
}
