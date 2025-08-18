use crate::components::property_selector::PropertySelector;
use crate::components::random_distribution_edit::RandomDistributionEdit;
use crate::components::Component;
use egui::{Grid, Ui};
use idct_game::curiosity::generator::property::CuriosityGeneratorProperty;
use idct_game::curiosity::property::definition::CuriosityPropertyDefinitions;

pub struct CuriosityGeneratorPropertyEdit<'a> {
    generator_property: &'a mut CuriosityGeneratorProperty,
    property_definitions: &'a CuriosityPropertyDefinitions,
}

impl<'a> CuriosityGeneratorPropertyEdit<'a> {
    pub fn new(
        generator_property: &'a mut CuriosityGeneratorProperty,
        property_definitions: &'a CuriosityPropertyDefinitions,
    ) -> Self {
        Self {
            generator_property,
            property_definitions,
        }
    }
}

impl Component for CuriosityGeneratorPropertyEdit<'_> {
    fn show(self, ui: &mut Ui) {
        Grid::new("curiosity_generator_property_edit")
            .striped(true)
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Property");
                PropertySelector::new(
                    &mut self.generator_property.property,
                    &self.property_definitions.definitions,
                    |property_def| &property_def.id,
                )
                .display(|property_id| property_id.as_ref())
                .id("curiosity_generator_property_edit_property_selector")
                .show(ui);
                ui.end_row();

                ui.label("Distribution");
                RandomDistributionEdit::new(&mut self.generator_property.random_distribution)
                    .show(ui);
                ui.end_row();
            });
    }
}
