use crate::data::curiosity::tag::definition::CuriosityTagDefinitions;
use crate::dimension::definition::DimensionDefinitions;
use crate::utils::traits::mergeable::Mergeable;
use curiosity::generator::CuriosityGenerators;
use curiosity::property::definition::CuriosityPropertyDefinitions;
use curiosity::tag::rules::CuriosityTagRules;
use serde::{Deserialize, Serialize};

pub mod curiosity;
pub mod pack;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GameData {
    #[serde(default, skip_serializing_if = "CuriosityTagDefinitions::is_empty")]
    pub curiosity_tag_definitions: CuriosityTagDefinitions,
    #[serde(default, skip_serializing_if = "CuriosityTagRules::is_empty")]
    pub curiosity_tag_rules: CuriosityTagRules,
    #[serde(
        default,
        skip_serializing_if = "CuriosityPropertyDefinitions::is_empty"
    )]
    pub curiosity_properties: CuriosityPropertyDefinitions,
    #[serde(default, skip_serializing_if = "DimensionDefinitions::is_empty")]
    pub dimensions: DimensionDefinitions,
    #[serde(default, skip_serializing_if = "CuriosityGenerators::is_empty")]
    pub curiosity_generators: CuriosityGenerators,
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
        self.curiosity_tag_definitions
            .merge(other.curiosity_tag_definitions);
        self.curiosity_tag_rules.merge(other.curiosity_tag_rules);
        self.curiosity_properties.merge(other.curiosity_properties);
        self.dimensions.merge(other.dimensions);
        self.curiosity_generators.merge(other.curiosity_generators);
    }
}
