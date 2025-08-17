use crate::error::GameResult;
use crate::utils::random::distribution::{RandomDistributionError, RandomizedDistribution};
use rand::Rng;
use rand_distr::Distribution;
use rand_distr::Normal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalDistribution {
    pub mean: f32,
    pub std_dev: f32,
}

impl Default for NormalDistribution {
    fn default() -> Self {
        Self {
            mean: 0.0,
            std_dev: 1.0,
        }
    }
}

impl NormalDistribution {
    pub fn new(mean: f32, std_dev: f32) -> Self {
        Self { mean, std_dev }
    }
}

impl RandomizedDistribution for NormalDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GameResult<f32> {
        Ok(Normal::new(self.mean, self.std_dev)
            .map_err(RandomDistributionError::from)?
            .sample(rng))
    }
}
