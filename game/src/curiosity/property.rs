use crate::curiosity::property::data::CuriosityPropertyData;
use crate::curiosity::property::id::CuriosityPropertyID;
use serde::{Deserialize, Serialize};

pub mod data;
pub mod definition;
pub mod id;
pub mod types;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuriosityProperty {
    pub id: CuriosityPropertyID,
    pub data: CuriosityPropertyData,
}
