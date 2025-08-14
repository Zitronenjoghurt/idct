use crate::components::Component;
use egui::Ui;
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
        ui.text_edit_singleline(self.tag_rule.identifier.as_mut());
    }
}
