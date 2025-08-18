use crate::components::tag_rule::tag_rules_edit::TagRulesEdit;
use crate::components::Component;
use crate::state::AppState;
use crate::windows::ViewWindow;
use egui::{Id, Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CuriosityTagsWindow {
    open: bool,
}

impl ViewWindow for CuriosityTagsWindow {
    fn id(&self) -> Id {
        Id::new("content_editor_curiosity_tags_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Curiosity Tags"
    }

    fn is_open(&self) -> bool {
        self.open
    }

    fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    fn render_content(&mut self, ui: &mut Ui, state: &mut AppState) {
        TagRulesEdit::new(
            &mut state.content_editor.edited_pack.data.tag_rules,
            &state.content_editor.edited_pack.data.curiosity_properties,
            &state.content_editor.context,
        )
        .show(ui);
    }

    fn resizable(&self) -> bool {
        false
    }
}
