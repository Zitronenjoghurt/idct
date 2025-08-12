use crate::curiosity::tag::rules::TagRules;
use crate::error::GameResult;
use crate::utils::traits::mergeable::Mergeable;
use crate::CORE_DATA_BYTES;
use serde::{Deserialize, Serialize};
use std::path::Path;

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

    pub fn load_from_bytes(&mut self, data: &[u8]) -> GameResult<()> {
        let data: Self = serde_json::from_slice(data)?;
        self.merge(data);
        Ok(())
    }

    pub fn load_from_core_data(&mut self) -> GameResult<()> {
        self.load_from_bytes(CORE_DATA_BYTES)
    }

    pub fn load_from_file(&mut self, path: &Path) -> GameResult<()> {
        let raw_data = std::fs::read(path)?;
        self.load_from_bytes(&raw_data)
    }
}

impl Mergeable for GameData {
    fn merge(&mut self, other: Self) {
        self.tag_rules.merge(other.tag_rules);
    }
}
