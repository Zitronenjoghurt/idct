use crate::components::curiosity::tag::curiosity_tag_rule_edit::CuriosityTagRuleEdit;
use crate::components::list_edit::ListEdit;
use crate::components::Component;
use crate::systems::content_editor::context::ContentEditorContext;
use egui::Ui;
use idct_game::data::curiosity::property::definition::CuriosityPropertyDefinitions;
use idct_game::data::curiosity::tag::definition::CuriosityTagDefinitions;
use idct_game::data::curiosity::tag::rules::CuriosityTagRules;

pub struct CuriosityTagRulesEdit<'a> {
    tag_rules: &'a mut CuriosityTagRules,
    property_definitions: &'a CuriosityPropertyDefinitions,
    tag_definitions: &'a CuriosityTagDefinitions,
    context: &'a ContentEditorContext,
}

impl<'a> CuriosityTagRulesEdit<'a> {
    pub fn new(
        tag_rules: &'a mut CuriosityTagRules,
        property_definitions: &'a CuriosityPropertyDefinitions,
        tag_definitions: &'a CuriosityTagDefinitions,
        context: &'a ContentEditorContext,
    ) -> Self {
        Self {
            tag_rules,
            property_definitions,
            tag_definitions,
            context,
        }
    }
}

impl Component for CuriosityTagRulesEdit<'_> {
    fn show(self, ui: &mut Ui) {
        ListEdit::new(
            &mut self.tag_rules.rules,
            |tag_rule, ui| {
                CuriosityTagRuleEdit::new(
                    tag_rule,
                    self.property_definitions,
                    self.tag_definitions,
                    self.context,
                )
                .show(ui);
            },
            |tag_rule| tag_rule.tag_id.as_ref().to_string(),
        )
        .show(ui);
    }
}
