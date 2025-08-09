use crate::curiosity::generation::blueprint::CuriosityGenerationBlueprint;
use crate::curiosity::generation::layers::CuriosityGenerationLayer;
use crate::curiosity::Curiosity;
use crate::utils::random::seed::RandomSeed;

mod blueprint;
mod layers;

#[derive(Debug, Default)]
pub struct CuriosityGenerator {
    seed: RandomSeed,
    layers: Vec<Box<dyn CuriosityGenerationLayer>>,
}

impl CuriosityGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_seed(seed: RandomSeed) -> Self {
        Self {
            seed,
            ..Default::default()
        }
    }

    pub fn add(mut self, layer: Box<dyn CuriosityGenerationLayer>) -> Self {
        self.layers.push(layer);
        self
    }

    pub fn generate(&self) -> Curiosity {
        let mut rng = self.seed.create_rng();
        let mut blueprint = CuriosityGenerationBlueprint::new();

        self.layers
            .iter()
            .for_each(|layer| layer.apply(&mut blueprint, &mut rng));

        blueprint.build()
    }
}
