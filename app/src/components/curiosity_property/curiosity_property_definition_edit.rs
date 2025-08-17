use crate::components::Component;
use egui::{Grid, Ui};
use idct_game::curiosity::property::definition::CuriosityPropertyDefinition;
use idct_game::curiosity::property::types::CuriosityPropertyType;

pub struct CuriosityPropertyDefinitionEdit<'a> {
    definition: &'a mut CuriosityPropertyDefinition,
}

impl<'a> CuriosityPropertyDefinitionEdit<'a> {
    pub fn new(definition: &'a mut CuriosityPropertyDefinition) -> Self {
        Self { definition }
    }
}

impl Component for CuriosityPropertyDefinitionEdit<'_> {
    fn show(self, ui: &mut Ui) {
        Grid::new("curiosity_property_definition_edit")
            .striped(true)
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("ID");
                ui.text_edit_singleline(self.definition.id.as_mut());
                ui.end_row();

                ui.label("Type");
                egui::ComboBox::from_id_salt("property_type")
                    .selected_text(self.definition.property_type.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.definition.property_type,
                            CuriosityPropertyType::Normalized,
                            CuriosityPropertyType::Normalized.to_string(),
                        );
                    });
            });
    }
}
