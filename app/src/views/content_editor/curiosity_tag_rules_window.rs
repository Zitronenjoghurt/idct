use crate::components::curiosity::tag::curiosity_tag_rules_edit::CuriosityTagRulesEdit;
use crate::components::Component;
use crate::state::AppState;
use crate::windows::ViewWindow;
use egui::{Id, Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CuriosityTagRulesWindow {
    open: bool,
}

impl ViewWindow for CuriosityTagRulesWindow {
    fn id(&self) -> Id {
        Id::new("content_editor_curiosity_tag_rules_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Curiosity Tag Rules"
    }

    fn is_open(&self) -> bool {
        self.open
    }

    fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    fn render_content(&mut self, ui: &mut Ui, state: &mut AppState) {
        CuriosityTagRulesEdit::new(
            &mut state.content_editor.edited_pack.data.curiosity_tag_rules,
            &state.content_editor.edited_pack.data.curiosity_properties,
            &state
                .content_editor
                .edited_pack
                .data
                .curiosity_tag_definitions,
            &state.content_editor.context,
        )
        .show(ui);
    }

    fn resizable(&self) -> bool {
        false
    }
}
