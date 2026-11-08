use crate::follow_camera::FollowCamera;
use crate::follow_camera::FollowCameraTarget;
use crate::follow_camera::FollowCameras;
use crate::follow_camera::camera_entity;
use crate::world_generation::WorldGeneratable;
use crate::world_generation::WorldGenerator;
use bevy::asset::asset_value;
use bevy::color::Color;
use bevy::ecs::component::Component;
use bevy::ecs::name::Name;
use bevy::image::TranscodeFormat;
use bevy::light::PointLight;
use bevy::mesh::Mesh3d;
use bevy::pbr::MeshMaterial3d;
use bevy::pbr::StandardMaterial;
use bevy::scene::bsn;
use bevy::shape::Sphere;
use bevy::transform::components::Transform;
use rand::Rng;
use rand::RngExt;
use rand::SeedableRng;
use rand::rngs::StdRng;

#[derive(Component, Clone, Default)]
pub struct Sun {
    generator: SunGenerator,
}

impl WorldGeneratable for Sun {}

#[derive(Clone, Default)]
pub struct SunGenerator {
    size: f32,
}

impl WorldGenerator for SunGenerator {
    type Generatable = Sun;

    fn new_from_seed(seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        Self {
            size: rng.random_range(100..200) as f32,
        }
    }

    fn generate(self) -> impl bevy::scene::prelude::Scene {
        let generator = self;
        bsn! {
            Name("Sun")
            Transform
            MeshMaterial3d::<StandardMaterial>(asset_value(Color::srgb(1., 0.1, 0.1)))
            Mesh3d(asset_value(Sphere::new(generator.size)))
            FollowCameraTarget::new(generator.size * 2.)
            PointLight {
                color: Color::srgb(1.,1.,0.1),
                intensity: 1_000_000_000.,
                range: 10_000.,
            }
            Sun { generator }
            FollowCameras [
                @{ camera_entity() }
            ]
        }
    }
}
