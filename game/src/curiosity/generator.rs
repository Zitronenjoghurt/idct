use crate::curiosity::generator::property::CuriosityGeneratorProperties;
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
