use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Dimension {
    #[default]
    DwarfSpaceAge,
}

impl Dimension {
    pub const ALL: [Dimension; 1] = [Dimension::DwarfSpaceAge];
}
