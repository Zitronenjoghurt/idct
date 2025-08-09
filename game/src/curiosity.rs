use crate::curiosity::properties::mass::Mass;
use crate::curiosity::properties::temperature::Temperature;

mod properties;

#[derive(Debug, Default)]
pub struct Curiosity {
    mass: Mass,
    temperature: Temperature,
}
