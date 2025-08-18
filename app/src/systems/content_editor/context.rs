use crate::systems::content_editor::actions::ContentEditorAction;
use idct_game::curiosity::tag::id::TagID;
use idct_game::data::GameData;
use std::cell::RefCell;

#[derive(Debug, Default)]
pub struct ContentEditorContext {
    action_queue: RefCell<Vec<ContentEditorAction>>,
    pub cached_tag_ids: Vec<TagID>,
}

impl ContentEditorContext {
    pub fn update(&mut self, data: &GameData) {
        self.update_tag_ids(data);
    }

    pub fn drain_actions(&self) -> impl Iterator<Item = ContentEditorAction> {
        if let Ok(mut queue) = self.action_queue.try_borrow_mut() {
            queue.drain(..).collect::<Vec<_>>().into_iter()
        } else {
            Vec::new().into_iter()
        }
    }

    pub fn rename_curiosity_property(&self, old: impl Into<String>, new: impl Into<String>) {
        self.push_action(ContentEditorAction::RenameCuriosityProperty {
            old: old.into(),
            new: new.into(),
        })
    }

    pub fn push_action(&self, action: ContentEditorAction) {
        if let Ok(mut queue) = self.action_queue.try_borrow_mut() {
            queue.push(action)
        }
    }

    fn update_tag_ids(&mut self, data: &GameData) {
        self.cached_tag_ids = data
            .tag_rules
            .rules
            .iter()
            .map(|rule| rule.id.clone())
            .collect();
    }
}
