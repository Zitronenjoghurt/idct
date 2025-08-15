use crate::curiosity::property::CuriosityProperty;
use crate::dimension::Dimension;
use serde::{Deserialize, Serialize};

pub mod generation;
pub mod property;
pub mod tag;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Curiosity {
    origin: Dimension,
    properties: Vec<CuriosityProperty>,
}
