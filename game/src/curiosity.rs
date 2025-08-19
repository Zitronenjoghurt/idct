use crate::curiosity::property::CuriosityProperty;
use crate::dimension::id::DimensionID;
use serde::{Deserialize, Serialize};

pub mod property;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Curiosity {
    origin: DimensionID,
    properties: Vec<CuriosityProperty>,
}
