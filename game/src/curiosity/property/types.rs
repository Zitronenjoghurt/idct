use serde::{Deserialize, Serialize};

pub mod normalized;

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum CuriosityPropertyType {
    #[default]
    Normalized,
}
