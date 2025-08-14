use crate::components::tag_rule_edit::TagRuleEdit;
use crate::components::Component;
use egui::Ui;
use idct_game::curiosity::tag::rules::TagRules;

pub struct TagRulesEdit<'a> {
    tag_rules: &'a mut TagRules,
}

impl<'a> TagRulesEdit<'a> {
    pub fn new(tag_rules: &'a mut TagRules) -> Self {
        Self { tag_rules }
    }
}

impl Component for TagRulesEdit<'_> {
    fn show(self, ui: &mut Ui) {
        for tag_rule in self.tag_rules.rules.iter_mut() {
            TagRuleEdit::new(tag_rule).show(ui);
        }

        if ui.button("Add").clicked() {
            self.tag_rules.rules.push(Default::default());
        }
    }
}
