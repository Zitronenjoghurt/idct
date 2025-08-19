use crate::components::curiosity::tag::curiosity_tag_rule_property_range_edit::CuriosityTagRulePropertyRangeEdit;
use crate::components::curiosity::tag::curiosity_tag_rule_tag_relation_edit::CuriosityTagRuleTagRelationEdit;
use crate::components::list_edit::ListEdit;
use crate::components::property_selector::PropertySelector;
use crate::components::Component;
use crate::systems::content_editor::context::ContentEditorContext;
use egui::{Grid, Ui};
use idct_game::data::curiosity::property::definition::CuriosityPropertyDefinitions;
use idct_game::data::curiosity::tag::definition::CuriosityTagDefinitions;
use idct_game::data::curiosity::tag::rules::CuriosityTagRule;

pub struct CuriosityTagRuleEdit<'a> {
    tag_rule: &'a mut CuriosityTagRule,
    property_definitions: &'a CuriosityPropertyDefinitions,
    tag_definitions: &'a CuriosityTagDefinitions,
    context: &'a ContentEditorContext,
}

impl<'a> CuriosityTagRuleEdit<'a> {
    pub fn new(
        tag_rule: &'a mut CuriosityTagRule,
        property_definitions: &'a CuriosityPropertyDefinitions,
        tag_definitions: &'a CuriosityTagDefinitions,
        context: &'a ContentEditorContext,
    ) -> Self {
        Self {
            tag_rule,
            property_definitions,
            tag_definitions,
            context,
        }
    }
}

impl Component for CuriosityTagRuleEdit<'_> {
    fn show(self, ui: &mut Ui) {
        Grid::new("tag_rule_edit")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                ui.label("ID");
                PropertySelector::new(
                    &mut self.tag_rule.tag_id,
                    &self.tag_definitions.definitions,
                    |tag_def| &tag_def.id,
                )
                .display(|tag_id| tag_id.as_ref())
                .condition(|tag_def| !tag_def.id.is_empty())
                .id("tag_rule_edit_tag_id_selector")
                .show(ui);
                ui.end_row();
                ui.end_row();

                ui.label("Property Ranges");
                ui.vertical(|ui| {
                    ListEdit::new(
                        &mut self.tag_rule.properties,
                        |property_range, ui| {
                            CuriosityTagRulePropertyRangeEdit::new(
                                property_range,
                                self.property_definitions,
                            )
                            .show(ui);
                        },
                        |property_range| property_range.property.as_ref().to_string(),
                    )
                    .id("tag_rule_edit_list_edit_property_ranges")
                    .max_height(100.0)
                    .max_width(200.0)
                    .show(ui);
                });
                ui.end_row();

                ui.label("Positive Relations");
                ui.vertical(|ui| {
                    ListEdit::new(
                        &mut self.tag_rule.positive,
                        |tag_relation, ui| {
                            CuriosityTagRuleTagRelationEdit::new(tag_relation, self.context)
                                .id("tag_rule_edit_positive_relations")
                                .parent_tag_id(&self.tag_rule.tag_id)
                                .show(ui);
                        },
                        |tag_relation| tag_relation.tag.as_ref().to_string(),
                    )
                    .id("tag_rule_edit_list_edit_positive_relations")
                    .max_height(100.0)
                    .max_width(200.0)
                    .show(ui);
                });
                ui.end_row();

                ui.label("Negative Relations");
                ui.vertical(|ui| {
                    ListEdit::new(
                        &mut self.tag_rule.negative,
                        |tag_relation, ui| {
                            CuriosityTagRuleTagRelationEdit::new(tag_relation, self.context)
                                .id("tag_rule_edit_negative_relations")
                                .parent_tag_id(&self.tag_rule.tag_id)
                                .show(ui);
                        },
                        |tag_relation| tag_relation.tag.as_ref().to_string(),
                    )
                    .id("tag_rule_edit_list_edit_negative_relations")
                    .max_height(100.0)
                    .max_width(200.0)
                    .show(ui);
                });
                ui.end_row();
            });
    }
}
