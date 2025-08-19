use crate::curiosity::property::data::CuriosityPropertyData;
use crate::data::curiosity::property::id::CuriosityPropertyID;
use serde::{Deserialize, Serialize};

pub mod data;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuriosityProperty {
    pub id: CuriosityPropertyID,
    pub data: CuriosityPropertyData,
}
