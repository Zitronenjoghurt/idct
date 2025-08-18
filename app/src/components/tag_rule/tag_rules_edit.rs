use crate::components::list_edit::ListEdit;
use crate::components::tag_rule::tag_rule_edit::TagRuleEdit;
use crate::components::Component;
use crate::systems::content_editor::context::ContentEditorContext;
use egui::Ui;
use idct_game::curiosity::property::definition::CuriosityPropertyDefinitions;
use idct_game::curiosity::tag::rules::TagRules;

pub struct TagRulesEdit<'a> {
    tag_rules: &'a mut TagRules,
    property_definitions: &'a CuriosityPropertyDefinitions,
    context: &'a ContentEditorContext,
}

impl<'a> TagRulesEdit<'a> {
    pub fn new(
        tag_rules: &'a mut TagRules,
        property_definitions: &'a CuriosityPropertyDefinitions,
        context: &'a ContentEditorContext,
    ) -> Self {
        Self {
            tag_rules,
            property_definitions,
            context,
        }
    }
}

impl Component for TagRulesEdit<'_> {
    fn show(self, ui: &mut Ui) {
        ListEdit::new(
            &mut self.tag_rules.rules,
            |tag_rule, ui| {
                TagRuleEdit::new(tag_rule, self.property_definitions, self.context).show(ui);
            },
            |tag_rule| tag_rule.id.as_ref().to_string(),
        )
        .show(ui);
    }
}
