use crate::data::curiosity::tag::id::CuriosityTagID;
use crate::utils::traits::mergeable::Mergeable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CuriosityTagDefinitions {
    pub definitions: Vec<CuriosityTagDefinition>,
}

impl CuriosityTagDefinitions {
    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }

    pub fn id_exists(&self, id: &str) -> bool {
        self.definitions
            .iter()
            .any(|definition| definition.id.as_ref() == id)
    }

    pub fn find_by_id(&self, id: &str) -> Option<&CuriosityTagDefinition> {
        self.definitions
            .iter()
            .find(|definition| definition.id.as_ref() == id)
    }

    pub fn find_by_id_mut(&mut self, id: &str) -> Option<&mut CuriosityTagDefinition> {
        self.definitions
            .iter_mut()
            .find(|definition| definition.id.as_ref() == id)
    }
}

impl Mergeable for CuriosityTagDefinitions {
    fn merge(&mut self, other: Self) {
        self.definitions.extend(other.definitions);
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CuriosityTagDefinition {
    pub id: CuriosityTagID,
}
