use crate::curiosity::generation::blueprint::CuriosityGenerationBlueprint;
use rand::prelude::StdRng;
use std::fmt::Debug;

pub mod origin;

pub trait CuriosityGenerationLayer: Debug {
    fn apply(&self, blueprint: &mut CuriosityGenerationBlueprint, rng: &mut StdRng);
}
