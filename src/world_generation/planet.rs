use crate::camera::CameraTarget;
use crate::orbiting::Orbiting;
use crate::sphere_math::move_in_direction;
use crate::world_generation::WorldGeneratable;
use bevy::asset::asset_value;
use bevy::color::Color;
use bevy::ecs::component::Component;
use bevy::ecs::hierarchy::Children;
use bevy::math::Vec3;
use bevy::mesh::Mesh3d;
use bevy::pbr::MeshMaterial3d;
use bevy::pbr::StandardMaterial;
use bevy::scene::Scene;
use bevy::scene::bsn;
use bevy::shape::Cuboid;
use bevy::shape::Sphere;
use bevy::sprite::Text2d;
use bevy::transform::components::Transform;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

#[derive(Component, Clone, Default)]
pub struct Planet;

pub struct PlanetGenerator {
    size: f32,
}

impl WorldGeneratable for PlanetGenerator {
    fn new_from_seed(seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        Self {
            size: rng.next_u32() as f32,
        }
    }

    fn generate(self) -> impl Scene {
        let mut sectors_positions = Vec::new();
        let radius = self.size;
        let mut last_position = Vec3::new(radius, 0., 0.);

        for _ in 0..10 {
            sectors_positions.push(last_position);
            last_position = move_in_direction(&last_position, &Vec3::new(0., -1., 0.), 1.);
        }

        let sectors = sectors_positions
            .into_iter()
            .map(|position| {
                bsn! {
                    Mesh3d(asset_value(Cuboid::new(1.0, 1.0, 1.0)))
                    MeshMaterial3d::<StandardMaterial>(asset_value(Color::srgb(0.75, 0.75, 0.8)))
                    Transform::from_translation(position)
                }
            })
            .collect::<Vec<_>>();

        bsn! {
            Mesh3d(asset_value(Sphere::new(radius)))
            MeshMaterial3d::<StandardMaterial>(asset_value(Color::srgb(0.2, 0.5, 1.0)))
            Transform
            Planet
            CameraTarget{radius: 40.}
            Orbiting()
            Children [
                {sectors}
                --
                Transform::from_xyz(0., radius +1., 0.)
                Text2d("N")
            ]
        }
    }
}
