use crate::components::curiosity::generator::curiosity_generator_properties_edit::CuriosityGeneratorPropertiesEdit;
use crate::components::Component;
use egui::{Grid, Ui};
use idct_game::curiosity::generator::CuriosityGenerator;
use idct_game::curiosity::property::definition::CuriosityPropertyDefinitions;

pub struct CuriosityGeneratorEdit<'a> {
    generator: &'a mut CuriosityGenerator,
    property_definitions: &'a CuriosityPropertyDefinitions,
}

impl<'a> CuriosityGeneratorEdit<'a> {
    pub fn new(
        generator: &'a mut CuriosityGenerator,
        property_definitions: &'a CuriosityPropertyDefinitions,
    ) -> Self {
        Self {
            generator,
            property_definitions,
        }
    }
}

impl Component for CuriosityGeneratorEdit<'_> {
    fn show(self, ui: &mut Ui) {
        Grid::new("curiosity_generator_edit")
            .striped(true)
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Properties");
                CuriosityGeneratorPropertiesEdit::new(
                    &mut self.generator.properties,
                    self.property_definitions,
                )
                .show(ui);
                ui.end_row();
            });
    }
}
