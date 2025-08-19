use crate::data::curiosity::property::id::CuriosityPropertyID;
use crate::data::curiosity::tag::id::CuriosityTagID;
use crate::utils::traits::mergeable::Mergeable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CuriosityTagRules {
    pub rules: Vec<CuriosityTagRule>,
}

impl CuriosityTagRules {
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}

impl Mergeable for CuriosityTagRules {
    fn merge(&mut self, other: Self) {
        self.rules.merge(other.rules);
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CuriosityTagRule {
    pub tag_id: CuriosityTagID,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<CuriosityTagRulePropertyRange>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub positive: Vec<CuriosityTagRuleTagRelation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub negative: Vec<CuriosityTagRuleTagRelation>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CuriosityTagRulePropertyRange {
    pub property: CuriosityPropertyID,
    pub min: f32,
    pub max: f32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CuriosityTagRuleTagRelation {
    pub tag: CuriosityTagID,
    pub factor: f32,
}
