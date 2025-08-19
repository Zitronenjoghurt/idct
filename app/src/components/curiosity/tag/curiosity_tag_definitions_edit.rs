use crate::components::curiosity::tag::curiosity_tag_definition_edit::CuriosityTagDefinitionEdit;
use crate::components::list_edit::ListEdit;
use crate::components::Component;
use crate::systems::content_editor::context::ContentEditorContext;
use egui::Ui;
use idct_game::data::curiosity::tag::definition::CuriosityTagDefinitions;

pub struct CuriosityTagDefinitionsEdit<'a> {
    definitions: &'a mut CuriosityTagDefinitions,
    context: &'a ContentEditorContext,
}

impl<'a> CuriosityTagDefinitionsEdit<'a> {
    pub fn new(
        definitions: &'a mut CuriosityTagDefinitions,
        context: &'a ContentEditorContext,
    ) -> Self {
        Self {
            definitions,
            context,
        }
    }
}

impl Component for CuriosityTagDefinitionsEdit<'_> {
    fn show(self, ui: &mut Ui) {
        ListEdit::new(
            &mut self.definitions.definitions,
            |definition, ui| {
                CuriosityTagDefinitionEdit::new(definition, self.context).show(ui);
            },
            |definition| definition.id.as_ref().to_string(),
        )
        .show(ui);
    }
}
