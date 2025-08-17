use crate::curiosity::generator::property::CuriosityGeneratorProperties;
use crate::dimension::id::DimensionID;
use serde::{Deserialize, Serialize};

pub mod property;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CuriosityGenerators {
    pub generators: Vec<CuriosityGenerator>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CuriosityGenerator {
    pub dimension: Option<DimensionID>,
    pub properties: CuriosityGeneratorProperties,
}
