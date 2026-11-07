use crate::world_generation::WorldGeneratable;
use crate::world_generation::planet::PlanetGenerator;
use bevy::ecs::component::Component;
use bevy::ecs::hierarchy::Children;
use bevy::scene::Scene;
use bevy::scene::bsn;
use rand::Rng;
use rand::RngExt;
use rand::SeedableRng;
use rand::rngs::StdRng;

#[derive(Component, Clone, Default)]
pub struct SolarSystem;
pub struct SolarSystemGenerator {
    planet_seeds: Vec<u64>,
}

impl WorldGeneratable for SolarSystemGenerator {
    fn new_from_seed(seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let planet_amout = rng.random_range(0..10);
        let planet_seeds = (0..planet_amout).map(|_| rng.next_u64()).collect();
        Self { planet_seeds }
    }

    fn generate(self) -> impl Scene {
        let planets = self
            .planet_seeds
            .clone()
            .into_iter()
            .map(|s| PlanetGenerator::new_from_seed(s.clone()).generate())
            .collect::<Vec<_>>();
        bsn! {
            SolarSystem
            Children[
                { planets }
            ]
        }
    }
}
