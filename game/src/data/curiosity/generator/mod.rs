use crate::data::curiosity::generator::property::CuriosityGeneratorProperties;
use crate::utils::traits::mergeable::Mergeable;
use serde::{Deserialize, Serialize};

pub mod property;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CuriosityGenerators {
    pub generators: Vec<CuriosityGenerator>,
}

impl CuriosityGenerators {
    pub fn is_empty(&self) -> bool {
        self.generators.is_empty()
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CuriosityGenerator {
    pub properties: CuriosityGeneratorProperties,
    pub name: String,
}

impl Mergeable for CuriosityGenerators {
    fn merge(&mut self, other: Self) {
        self.generators.extend(other.generators);
    }
}
