use crate::data::curiosity::property::types;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CuriosityPropertyData {
    Normalized(types::normalized::NormalizedCuriosityProperty),
}
