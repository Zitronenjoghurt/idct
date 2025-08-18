use crate::components::curiosity::property::curiosity_property_definition_edit::CuriosityPropertyDefinitionEdit;
use crate::components::list_edit::ListEdit;
use crate::components::Component;
use crate::systems::content_editor::context::ContentEditorContext;
use egui::Ui;
use idct_game::curiosity::property::definition::CuriosityPropertyDefinitions;

pub struct CuriosityPropertyDefinitionsEdit<'a> {
    definitions: &'a mut CuriosityPropertyDefinitions,
    context: &'a ContentEditorContext,
}

impl<'a> CuriosityPropertyDefinitionsEdit<'a> {
    pub fn new(
        definitions: &'a mut CuriosityPropertyDefinitions,
        context: &'a ContentEditorContext,
    ) -> Self {
        Self {
            definitions,
            context,
        }
    }
}

impl Component for CuriosityPropertyDefinitionsEdit<'_> {
    fn show(self, ui: &mut Ui) {
        ListEdit::new(
            &mut self.definitions.definitions,
            |definition, ui| {
                CuriosityPropertyDefinitionEdit::new(definition, self.context).show(ui);
            },
            |definition| definition.id.as_ref().to_string(),
        )
        .show(ui);
    }
}
