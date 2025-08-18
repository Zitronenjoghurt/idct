use crate::components::dimension::dimension_definition_edit::DimensionDefinitionEdit;
use crate::components::list_edit::ListEdit;
use crate::components::Component;
use egui::Ui;
use idct_game::dimension::definition::DimensionDefinitions;

pub struct DimensionDefinitionsEdit<'a> {
    definitions: &'a mut DimensionDefinitions,
}

impl<'a> DimensionDefinitionsEdit<'a> {
    pub fn new(definitions: &'a mut DimensionDefinitions) -> Self {
        Self { definitions }
    }
}

impl Component for DimensionDefinitionsEdit<'_> {
    fn show(self, ui: &mut Ui) {
        ListEdit::new(
            &mut self.definitions.definitions,
            |definition, ui| {
                DimensionDefinitionEdit::new(definition).show(ui);
            },
            |definition| definition.id.as_ref().to_string(),
        )
        .show(ui);
    }
}
