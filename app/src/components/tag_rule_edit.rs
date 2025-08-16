use crate::components::Component;
use egui::{Grid, Ui};
use idct_game::curiosity::tag::rules::TagRule;

pub struct TagRuleEdit<'a> {
    tag_rule: &'a mut TagRule,
}

impl<'a> TagRuleEdit<'a> {
    pub fn new(tag_rule: &'a mut TagRule) -> Self {
        Self { tag_rule }
    }
}

impl Component for TagRuleEdit<'_> {
    fn show(self, ui: &mut Ui) {
        Grid::new("tag_rule_edit").num_columns(2).show(ui, |ui| {
            ui.label("Identifier");
            ui.text_edit_singleline(self.tag_rule.id.as_mut());
            ui.end_row();
        });
    }
}
