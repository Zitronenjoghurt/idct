use crate::components::curiosity::generator::curiosity_generator_property_edit::CuriosityGeneratorPropertyEdit;
use crate::components::list_edit::ListEdit;
use crate::components::Component;
use egui::Ui;
use idct_game::curiosity::generator::property::CuriosityGeneratorProperties;
use idct_game::curiosity::property::definition::CuriosityPropertyDefinitions;

pub struct CuriosityGeneratorPropertiesEdit<'a> {
    generator_properties: &'a mut CuriosityGeneratorProperties,
    property_definitions: &'a CuriosityPropertyDefinitions,
}

impl<'a> CuriosityGeneratorPropertiesEdit<'a> {
    pub fn new(
        generator_properties: &'a mut CuriosityGeneratorProperties,
        property_definitions: &'a CuriosityPropertyDefinitions,
    ) -> Self {
        Self {
            generator_properties,
            property_definitions,
        }
    }
}

impl Component for CuriosityGeneratorPropertiesEdit<'_> {
    fn show(self, ui: &mut Ui) {
        ListEdit::new(
            &mut self.generator_properties.properties,
            |curiosity_generator_property, ui| {
                CuriosityGeneratorPropertyEdit::new(
                    curiosity_generator_property,
                    self.property_definitions,
                )
                .show(ui);
            },
            |curiosity_generator_property| {
                curiosity_generator_property.property.as_ref().to_string()
            },
        )
        .show(ui);
    }
}
