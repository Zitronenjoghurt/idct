use crate::systems::content_editor::actions::ContentEditorAction;
use crate::systems::content_editor::context::ContentEditorContext;
use idct_game::data::pack::DataPack;
use idct_game::error::GameResult;
use serde::{Deserialize, Serialize};

mod actions;
pub mod context;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ContentEditor {
    pub edited_pack: DataPack,
    #[serde(skip, default)]
    pub context: ContentEditorContext,
}

impl ContentEditor {
    pub fn update(&mut self) {
        self.context.update(&self.edited_pack.data);

        let actions = self.context.drain_actions().collect::<Vec<_>>();
        for action in actions {
            self.process_action(action);
        }
    }

    pub fn restore_core_data(&mut self) -> GameResult<()> {
        self.edited_pack = DataPack::from_core_data()?;
        Ok(())
    }

    fn process_action(&mut self, action: ContentEditorAction) {
        match action {
            ContentEditorAction::RenameCuriosityProperty { old, new } => {
                self.rename_curiosity_property(old, new)
            }
            ContentEditorAction::RenameCuriosityTag { old, new } => {
                self.rename_curiosity_tag(old, new)
            }
        }
    }

    fn rename_curiosity_property(&mut self, old: String, new: String) {
        if new.is_empty() {
            return;
        }
        if self.edited_pack.data.curiosity_properties.id_exists(&new) {
            return;
        }

        let Some(curiosity_property) = self
            .edited_pack
            .data
            .curiosity_properties
            .find_by_id_mut(&old)
        else {
            return;
        };
        curiosity_property.id.set(&new);

        for tag_rule in self.edited_pack.data.curiosity_tag_rules.rules.iter_mut() {
            for property_range in tag_rule.properties.iter_mut() {
                if property_range.property.as_ref() == old {
                    property_range.property.set(&new);
                }
            }
        }

        for generator in self
            .edited_pack
            .data
            .curiosity_generators
            .generators
            .iter_mut()
        {
            for generator_property in generator.properties.properties.iter_mut() {
                if generator_property.property.as_ref() == old {
                    generator_property.property.set(&new);
                }
            }
        }
    }

    fn rename_curiosity_tag(&mut self, old: String, new: String) {
        if new.is_empty() {
            return;
        }
        if self
            .edited_pack
            .data
            .curiosity_tag_definitions
            .id_exists(&new)
        {
            return;
        }

        let Some(curiosity_tag) = self
            .edited_pack
            .data
            .curiosity_tag_definitions
            .find_by_id_mut(&old)
        else {
            return;
        };
        curiosity_tag.id.set(&new);

        for tag_rule in self.edited_pack.data.curiosity_tag_rules.rules.iter_mut() {
            if tag_rule.tag_id.as_ref() == old {
                tag_rule.tag_id.set(&new);
            }
        }
    }
}
