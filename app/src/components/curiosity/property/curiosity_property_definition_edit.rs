use crate::components::Component;
use crate::systems::content_editor::context::ContentEditorContext;
use egui::{Grid, Ui};
use idct_game::data::curiosity::property::definition::CuriosityPropertyDefinition;
use idct_game::data::curiosity::property::types::CuriosityPropertyType;

pub struct CuriosityPropertyDefinitionEdit<'a> {
    definition: &'a mut CuriosityPropertyDefinition,
    context: &'a ContentEditorContext,
}

impl<'a> CuriosityPropertyDefinitionEdit<'a> {
    pub fn new(
        definition: &'a mut CuriosityPropertyDefinition,
        context: &'a ContentEditorContext,
    ) -> Self {
        Self {
            definition,
            context,
        }
    }
}

impl Component for CuriosityPropertyDefinitionEdit<'_> {
    fn show(self, ui: &mut Ui) {
        Grid::new("curiosity_property_definition_edit")
            .striped(true)
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("ID");
                let mut id = self.definition.id.as_ref().to_string();
                if ui.text_edit_singleline(&mut id).changed() {
                    self.context
                        .rename_curiosity_property(self.definition.id.as_ref(), id);
                };
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
