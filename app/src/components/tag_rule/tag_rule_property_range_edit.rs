use crate::components::property_selector::PropertySelector;
use crate::components::Component;
use egui::{Grid, Ui};
use idct_game::curiosity::property::definition::CuriosityPropertyDefinitions;
use idct_game::curiosity::property::types::CuriosityPropertyType;
use idct_game::curiosity::tag::rules::TagRulePropertyRange;

pub struct TagRulePropertyRangeEdit<'a> {
    property_range: &'a mut TagRulePropertyRange,
    property_definitions: &'a CuriosityPropertyDefinitions,
}

impl<'a> TagRulePropertyRangeEdit<'a> {
    pub fn new(
        property_range: &'a mut TagRulePropertyRange,
        property_definitions: &'a CuriosityPropertyDefinitions,
    ) -> Self {
        Self {
            property_range,
            property_definitions,
        }
    }
}

impl Component for TagRulePropertyRangeEdit<'_> {
    fn show(self, ui: &mut Ui) {
        Grid::new("tag_rule_property_range_edit")
            .striped(true)
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Property");
                PropertySelector::new(
                    &mut self.property_range.property,
                    &self.property_definitions.definitions,
                    |property_def| &property_def.id,
                    |property_id| property_id.as_ref(),
                )
                .condition(|property_def| {
                    property_def.property_type == CuriosityPropertyType::Normalized
                })
                .id("property_range_property_selector")
                .show(ui);
                ui.end_row();

                let mut min = self.property_range.min;
                let mut max = self.property_range.max;

                ui.label("Min");
                ui.add(
                    egui::Slider::new(&mut min, 0.0..=max)
                        .fixed_decimals(2)
                        .text(""),
                );
                ui.end_row();

                ui.label("Max");
                ui.add(
                    egui::Slider::new(&mut max, min..=1.0)
                        .fixed_decimals(2)
                        .text(""),
                );
                ui.end_row();

                self.property_range.min = min;
                self.property_range.max = max;
            });
    }
}
