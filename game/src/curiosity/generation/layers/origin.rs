use crate::curiosity::generation::blueprint::CuriosityGenerationBlueprint;
use crate::curiosity::generation::layers::CuriosityGenerationLayer;
use crate::dimension::Dimension;
use rand::prelude::{IndexedRandom, StdRng};

#[derive(Debug, Default)]
pub struct OriginLayer {
    dimensions: Vec<Dimension>,
}

impl OriginLayer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_dimensions(dimensions: Vec<Dimension>) -> Self {
        Self { dimensions }
    }
}

impl CuriosityGenerationLayer for OriginLayer {
    fn apply(&self, blueprint: &mut CuriosityGenerationBlueprint, rng: &mut StdRng) {
        let dimension = if self.dimensions.is_empty() {
            Dimension::ALL.choose(rng)
        } else {
            self.dimensions.choose(rng)
        }
        .copied()
        .unwrap_or_default();

        blueprint.origin = dimension;
    }
}
