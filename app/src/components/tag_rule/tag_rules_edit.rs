use crate::components::list_edit::ListEdit;
use crate::components::tag_rule::tag_rule_edit::TagRuleEdit;
use crate::components::Component;
use egui::Ui;
use idct_game::curiosity::property::definition::CuriosityPropertyDefinitions;
use idct_game::curiosity::tag::rules::TagRules;

pub struct TagRulesEdit<'a> {
    tag_rules: &'a mut TagRules,
    property_definitions: &'a CuriosityPropertyDefinitions,
}

impl<'a> TagRulesEdit<'a> {
    pub fn new(
        tag_rules: &'a mut TagRules,
        property_definitions: &'a CuriosityPropertyDefinitions,
    ) -> Self {
        Self {
            tag_rules,
            property_definitions,
        }
    }
}

impl Component for TagRulesEdit<'_> {
    fn show(self, ui: &mut Ui) {
        ListEdit::new(&mut self.tag_rules.rules, |tag_rule, ui| {
            TagRuleEdit::new(tag_rule, self.property_definitions).show(ui);
        })
        .show(ui);
    }
}
