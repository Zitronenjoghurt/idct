use crate::components::list_edit::ListEdit;
use crate::components::tag_rule::tag_rule_property_range_edit::TagRulePropertyRangeEdit;
use crate::components::tag_rule::tag_rule_tag_relation_edit::TagRuleTagRelationEdit;
use crate::components::Component;
use egui::{Grid, Ui};
use idct_game::curiosity::property::definition::CuriosityPropertyDefinitions;
use idct_game::curiosity::tag::id::TagID;
use idct_game::curiosity::tag::rules::TagRule;

pub struct TagRuleEdit<'a> {
    tag_rule: &'a mut TagRule,
    property_definitions: &'a CuriosityPropertyDefinitions,
    cached_tag_ids: &'a [TagID],
}

impl<'a> TagRuleEdit<'a> {
    pub fn new(
        tag_rule: &'a mut TagRule,
        property_definitions: &'a CuriosityPropertyDefinitions,
        cached_tag_ids: &'a [TagID],
    ) -> Self {
        Self {
            tag_rule,
            property_definitions,
            cached_tag_ids,
        }
    }
}

impl Component for TagRuleEdit<'_> {
    fn show(self, ui: &mut Ui) {
        Grid::new("tag_rule_edit")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                ui.label("Identifier");
                ui.text_edit_singleline(self.tag_rule.id.as_mut());
                ui.end_row();

                ui.label("Property Ranges");
                ui.vertical(|ui| {
                    ListEdit::new(&mut self.tag_rule.properties, |property_range, ui| {
                        TagRulePropertyRangeEdit::new(property_range, self.property_definitions)
                            .show(ui);
                    })
                    .id("tag_rule_edit_list_edit_property_ranges")
                    .max_height(100.0)
                    .max_width(200.0)
                    .show(ui);
                });
                ui.end_row();

                ui.label("Positive Relations");
                ui.vertical(|ui| {
                    ListEdit::new(&mut self.tag_rule.positive, |tag_relation, ui| {
                        TagRuleTagRelationEdit::new(tag_relation, self.cached_tag_ids)
                            .parent_tag_id(&self.tag_rule.id)
                            .show(ui);
                    })
                    .id("tag_rule_edit_list_edit_positive_relations")
                    .max_height(100.0)
                    .max_width(200.0)
                    .show(ui);
                });
                ui.end_row();

                ui.label("Negative Relations");
                ui.vertical(|ui| {
                    ListEdit::new(&mut self.tag_rule.negative, |tag_relation, ui| {
                        TagRuleTagRelationEdit::new(tag_relation, self.cached_tag_ids)
                            .parent_tag_id(&self.tag_rule.id)
                            .show(ui);
                    })
                    .id("tag_rule_edit_list_edit_negative_relations")
                    .max_height(100.0)
                    .max_width(200.0)
                    .show(ui);
                });
                ui.end_row();
            });
    }
}
