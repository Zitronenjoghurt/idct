use crate::data::curiosity::property::id::CuriosityPropertyID;
use crate::data::curiosity::property::types::CuriosityPropertyType;
use crate::utils::traits::mergeable::Mergeable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CuriosityPropertyDefinitions {
    pub definitions: Vec<CuriosityPropertyDefinition>,
}

impl CuriosityPropertyDefinitions {
    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }

    pub fn id_exists(&self, id: &str) -> bool {
        self.definitions
            .iter()
            .any(|definition| definition.id.as_ref() == id)
    }

    pub fn find_by_id(&self, id: &str) -> Option<&CuriosityPropertyDefinition> {
        self.definitions
            .iter()
            .find(|definition| definition.id.as_ref() == id)
    }

    pub fn find_by_id_mut(&mut self, id: &str) -> Option<&mut CuriosityPropertyDefinition> {
        self.definitions
            .iter_mut()
            .find(|definition| definition.id.as_ref() == id)
    }
}

impl Mergeable for CuriosityPropertyDefinitions {
    fn merge(&mut self, other: Self) {
        self.definitions.extend(other.definitions);
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CuriosityPropertyDefinition {
    pub id: CuriosityPropertyID,
    pub property_type: CuriosityPropertyType,
}
