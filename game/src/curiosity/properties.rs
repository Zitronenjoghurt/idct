use serde::{Deserialize, Serialize};

pub mod mass;
pub mod temperature;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CuriosityProperty {
    Mass,
    Temperature,
}
