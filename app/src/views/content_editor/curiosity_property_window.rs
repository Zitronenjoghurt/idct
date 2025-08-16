use crate::components::curiosity_property::curiosity_property_definitions_edit::CuriosityPropertyDefinitionsEdit;
use crate::components::Component;
use crate::state::AppState;
use crate::windows::ViewWindow;
use egui::{Id, Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CuriosityPropertyWindow {
    open: bool,
}

impl ViewWindow for CuriosityPropertyWindow {
    fn id(&self) -> Id {
        Id::new("content_editor_curiosity_property_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Curiosity Properties"
    }

    fn is_open(&self) -> bool {
        self.open
    }

    fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    fn render_content(&mut self, ui: &mut Ui, state: &mut AppState) {
        CuriosityPropertyDefinitionsEdit::new(
            &mut state.content_editor.edited_pack.data.curiosity_properties,
        )
        .show(ui);
    }

    fn resizable(&self) -> bool {
        false
    }
}
