use crate::curiosity::property::definition::CuriosityPropertyDefinitions;
use crate::curiosity::tag::rules::TagRules;
use crate::dimension::definition::DimensionDefinitions;
use crate::utils::traits::mergeable::Mergeable;
use serde::{Deserialize, Serialize};

pub mod pack;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GameData {
    #[serde(default, skip_serializing_if = "TagRules::is_empty")]
    pub tag_rules: TagRules,
    #[serde(
        default,
        skip_serializing_if = "CuriosityPropertyDefinitions::is_empty"
    )]
    pub curiosity_properties: CuriosityPropertyDefinitions,
    #[serde(default, skip_serializing_if = "DimensionDefinitions::is_empty")]
    pub dimensions: DimensionDefinitions,
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
        self.curiosity_properties.merge(other.curiosity_properties);
        self.dimensions.merge(other.dimensions);
    }
}
