use crate::curiosity::generation::blueprint::CuriosityGenerationBlueprint;
use rand::prelude::StdRng;
use std::fmt::Debug;

mod origin;

pub trait CuriosityGenerationLayer: Debug {
    fn apply(&self, blueprint: &mut CuriosityGenerationBlueprint, rng: &mut StdRng);
}
