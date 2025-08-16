use crate::curiosity::property::id::CuriosityPropertyID;
use crate::curiosity::tag::id::TagID;
use crate::utils::traits::mergeable::Mergeable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TagRules {
    pub rules: Vec<TagRule>,
}

impl TagRules {
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}

impl Mergeable for TagRules {
    fn merge(&mut self, other: Self) {
        self.rules.merge(other.rules);
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TagRule {
    pub id: TagID,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<TagRulePropertyRange>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub positive: Vec<TagRuleTagRelation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub negative: Vec<TagRuleTagRelation>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TagRulePropertyRange {
    pub property: CuriosityPropertyID,
    pub min: f32,
    pub max: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagRuleTagRelation {
    pub tag: TagID,
    pub factor: f32,
}
