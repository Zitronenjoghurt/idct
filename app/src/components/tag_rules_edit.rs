use crate::components::list_edit::ListEdit;
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
        ListEdit::new(&mut self.tag_rules.rules, |tag_rule, ui| {
            TagRuleEdit::new(tag_rule).show(ui);
        })
        .show(ui);
    }
}
