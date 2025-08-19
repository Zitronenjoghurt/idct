use crate::components::Component;
use crate::systems::content_editor::context::ContentEditorContext;
use egui::{Grid, Ui};
use idct_game::data::curiosity::tag::definition::CuriosityTagDefinition;

pub struct CuriosityTagDefinitionEdit<'a> {
    definition: &'a mut CuriosityTagDefinition,
    context: &'a ContentEditorContext,
}

impl<'a> CuriosityTagDefinitionEdit<'a> {
    pub fn new(
        definition: &'a mut CuriosityTagDefinition,
        context: &'a ContentEditorContext,
    ) -> Self {
        Self {
            definition,
            context,
        }
    }
}

impl Component for CuriosityTagDefinitionEdit<'_> {
    fn show(self, ui: &mut Ui) {
        Grid::new("curiosity_tag_definition_edit")
            .striped(true)
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("ID");
                let mut id = self.definition.id.as_ref().to_string();
                if ui.text_edit_singleline(&mut id).changed() {
                    self.context
                        .rename_curiosity_tag(self.definition.id.as_ref(), id);
                };
                ui.end_row();
            });
    }
}
