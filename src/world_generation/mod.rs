use bevy::prelude::*;

pub mod planet;
pub mod solar_system;
pub mod sun;

pub trait WorldGeneratable {}

pub trait WorldGenerator {
    type Generatable;
    fn new_from_seed(seed: u64) -> Self;
    fn generate(self) -> impl Scene;
}
