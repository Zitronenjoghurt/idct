use crate::curiosity::Curiosity;
use crate::dimension::Dimension;

#[derive(Debug, Default)]
pub struct CuriosityGenerationBlueprint {
    pub origin: Dimension,
}

impl CuriosityGenerationBlueprint {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Curiosity {
        Curiosity {
            origin: self.origin,
            ..Default::default()
        }
    }
}
