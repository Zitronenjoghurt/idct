use crate::curiosity::property::id::CuriosityPropertyID;
use crate::curiosity::property::types::CuriosityPropertyType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CuriosityPropertyDefinitions {
    pub definitions: Vec<CuriosityPropertyDefinition>,
}

impl CuriosityPropertyDefinitions {
    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CuriosityPropertyDefinition {
    pub id: CuriosityPropertyID,
    pub property_type: CuriosityPropertyType,
}
