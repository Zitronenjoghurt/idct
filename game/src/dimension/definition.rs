use crate::dimension::id::DimensionID;
use crate::utils::traits::mergeable::Mergeable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DimensionDefinitions {
    pub definitions: Vec<DimensionDefinition>,
}

impl DimensionDefinitions {
    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DimensionDefinition {
    pub id: DimensionID,
}

impl Mergeable for DimensionDefinitions {
    fn merge(&mut self, other: Self) {
        self.definitions.extend(other.definitions);
    }
}
