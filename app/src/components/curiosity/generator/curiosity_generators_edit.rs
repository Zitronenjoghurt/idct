use crate::components::curiosity::generator::curiosity_generator_edit::CuriosityGeneratorEdit;
use crate::components::list_edit::ListEdit;
use crate::components::Component;
use egui::Ui;
use idct_game::data::curiosity::generator::CuriosityGenerators;
use idct_game::data::curiosity::property::definition::CuriosityPropertyDefinitions;

pub struct CuriosityGeneratorsEdit<'a> {
    generators: &'a mut CuriosityGenerators,
    property_definitions: &'a CuriosityPropertyDefinitions,
}

impl<'a> CuriosityGeneratorsEdit<'a> {
    pub fn new(
        generators: &'a mut CuriosityGenerators,
        property_definitions: &'a CuriosityPropertyDefinitions,
    ) -> Self {
        Self {
            generators,
            property_definitions,
        }
    }
}

impl Component for CuriosityGeneratorsEdit<'_> {
    fn show(self, ui: &mut Ui) {
        ListEdit::new(
            &mut self.generators.generators,
            |generator, ui| {
                CuriosityGeneratorEdit::new(generator, self.property_definitions).show(ui);
            },
            |generator| "".to_string(),
        )
        .show(ui);
    }
}
