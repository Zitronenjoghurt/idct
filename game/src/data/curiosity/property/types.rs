use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub mod normalized;

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum CuriosityPropertyType {
    #[default]
    Normalized,
}

impl Display for CuriosityPropertyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CuriosityPropertyType::Normalized => write!(f, "Normalized"),
        }
    }
}
