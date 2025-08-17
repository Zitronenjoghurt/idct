use crate::error::GameResult;
use rand::Rng;
use serde::{Deserialize, Serialize};
use thiserror::Error;

mod normal;
mod uniform;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RandomDistribution {
    Normal(normal::NormalDistribution),
    Uniform(uniform::UniformDistribution),
}

impl Default for RandomDistribution {
    fn default() -> Self {
        Self::Normal(normal::NormalDistribution::default())
    }
}

impl RandomizedDistribution for RandomDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GameResult<f32> {
        match self {
            Self::Normal(normal) => normal.sample(rng),
            Self::Uniform(uniform) => uniform.sample(rng),
        }
    }
}

pub trait RandomizedDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GameResult<f32>;
}

#[derive(Debug, Error)]
pub enum RandomDistributionError {
    #[error("Normal distribution: {0}")]
    Normal(#[from] rand_distr::NormalError),
}
