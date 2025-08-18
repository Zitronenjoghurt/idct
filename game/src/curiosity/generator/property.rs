use crate::curiosity::property::id::CuriosityPropertyID;
use crate::utils::random::distribution::RandomDistribution;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CuriosityGeneratorProperties {
    pub properties: Vec<CuriosityGeneratorProperty>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CuriosityGeneratorProperty {
    pub property: CuriosityPropertyID,
    pub random_distribution: RandomDistribution,
}
