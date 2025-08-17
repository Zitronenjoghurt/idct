use crate::components::Component;
use egui::{Grid, Ui};
use idct_game::dimension::definition::DimensionDefinition;

pub struct DimensionDefinitionEdit<'a> {
    definition: &'a mut DimensionDefinition,
}

impl<'a> DimensionDefinitionEdit<'a> {
    pub fn new(definition: &'a mut DimensionDefinition) -> Self {
        Self { definition }
    }
}

impl Component for DimensionDefinitionEdit<'_> {
    fn show(self, ui: &mut Ui) {
        Grid::new("dimension_definition_edit")
            .striped(true)
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("ID");
                ui.text_edit_singleline(self.definition.id.as_mut());
                ui.end_row();
            });
    }
}
