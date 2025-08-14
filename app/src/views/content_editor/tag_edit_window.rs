use crate::components::tag_rules_edit::TagRulesEdit;
use crate::components::Component;
use crate::state::AppState;
use crate::windows::ViewWindow;
use egui::{Id, Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TagEditWindow {
    open: bool,
}

impl ViewWindow for TagEditWindow {
    fn id(&self) -> Id {
        Id::new("content_editor_tag_edit_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Tag Editor"
    }

    fn is_open(&self) -> bool {
        self.open
    }

    fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    fn render_content(&mut self, ui: &mut Ui, state: &mut AppState) {
        TagRulesEdit::new(&mut state.content_editor_state.edited_data.tag_rules).show(ui);
    }
}
