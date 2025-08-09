use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Mass(f64);
