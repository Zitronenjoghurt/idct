use crate::error::GameResult;
use crate::utils::random::distribution::RandomizedDistribution;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniformDistribution {
    pub min: f32,
    pub max: f32,
}

impl Default for UniformDistribution {
    fn default() -> Self {
        Self { min: 0.0, max: 1.0 }
    }
}

impl UniformDistribution {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }
}

impl RandomizedDistribution for UniformDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GameResult<f32> {
        let min = if self.min < self.max {
            self.min
        } else {
            self.max
        };
        Ok(rng.random_range(min..=self.max))
    }
}
