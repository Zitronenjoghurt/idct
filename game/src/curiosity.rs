use crate::curiosity::properties::mass::Mass;
use crate::curiosity::properties::temperature::Temperature;
use crate::dimension::Dimension;
use serde::{Deserialize, Serialize};

pub mod generation;
mod properties;
pub mod tag;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Curiosity {
    origin: Dimension,
    mass: Mass,
    temperature: Temperature,
}
