use crate::components::curiosity_property::curiosity_property_definition_edit::CuriosityPropertyDefinitionEdit;
use crate::components::list_edit::ListEdit;
use crate::components::Component;
use egui::Ui;
use idct_game::curiosity::property::definition::CuriosityPropertyDefinitions;

pub struct CuriosityPropertyDefinitionsEdit<'a> {
    definitions: &'a mut CuriosityPropertyDefinitions,
}

impl<'a> CuriosityPropertyDefinitionsEdit<'a> {
    pub fn new(definitions: &'a mut CuriosityPropertyDefinitions) -> Self {
        Self { definitions }
    }
}

impl Component for CuriosityPropertyDefinitionsEdit<'_> {
    fn show(self, ui: &mut Ui) {
        ListEdit::new(&mut self.definitions.definitions, |definition, ui| {
            CuriosityPropertyDefinitionEdit::new(definition).show(ui);
        })
        .show(ui);
    }
}
