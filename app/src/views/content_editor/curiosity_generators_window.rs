use crate::components::curiosity::generator::curiosity_generators_edit::CuriosityGeneratorsEdit;
use crate::components::Component;
use crate::state::AppState;
use crate::windows::ViewWindow;
use egui::{Id, Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CuriosityGeneratorsWindow {
    open: bool,
}

impl ViewWindow for CuriosityGeneratorsWindow {
    fn id(&self) -> Id {
        Id::new("content_editor_curiosity_generators_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Curiosity Generators"
    }

    fn is_open(&self) -> bool {
        self.open
    }

    fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    fn render_content(&mut self, ui: &mut Ui, state: &mut AppState) {
        CuriosityGeneratorsEdit::new(
            &mut state.content_editor.edited_pack.data.curiosity_generators,
            &state.content_editor.edited_pack.data.curiosity_properties,
        )
        .show(ui);
    }

    fn resizable(&self) -> bool {
        false
    }
}
