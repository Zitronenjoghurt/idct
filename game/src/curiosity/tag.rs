use serde::{Deserialize, Serialize};

pub mod rules;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TagID(String);
