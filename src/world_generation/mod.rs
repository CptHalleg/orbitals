use bevy::prelude::*;
use rand::RngExt;
use rand::{SeedableRng, rngs::StdRng};

use crate::Orbiting;
use crate::camera::OrbitCameraTargetable;
use crate::sphere_math::move_in_direction;
use rand::Rng;

pub mod planet;
pub mod solar_system;
pub mod sun;

pub trait WorldGeneratable {
    fn new_from_seed(seed: u64) -> Self;
    fn generate(self) -> impl Scene;
}
