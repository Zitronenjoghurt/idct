use crate::curiosity::tag::rules::TagRules;
use crate::utils::traits::mergeable::Mergeable;
use serde::{Deserialize, Serialize};

pub mod pack;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GameData {
    #[serde(default, skip_serializing_if = "TagRules::is_empty")]
    pub tag_rules: TagRules,
}

impl GameData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

impl Mergeable for GameData {
    fn merge(&mut self, other: Self) {
        self.tag_rules.merge(other.tag_rules);
    }
}
