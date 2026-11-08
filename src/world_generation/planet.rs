use crate::follow_camera::FollowCameraTarget;
use crate::orbiting::Orbit;
use crate::orbiting::Orbiting;
use crate::sphere_math::move_in_direction;
use crate::world_generation::WorldGeneratable;
use crate::world_generation::WorldGenerator;
use bevy::asset::asset_value;
use bevy::color::Color;
use bevy::ecs::component::Component;
use bevy::ecs::hierarchy::Children;
use bevy::ecs::name::Name;
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
use rand::RngExt;
use rand::SeedableRng;
use rand::rngs::StdRng;
use std::f32::consts::TAU;
use std::ops::Range;

#[derive(Component, Clone, Default)]
pub struct Planet {
    generator: PlanetGenerator,
}

impl WorldGeneratable for Planet {}

#[derive(Clone, Default)]
pub struct PlanetGenerator {
    size: f32,
    orbit: Orbit,
    moons: Vec<MoonGenerator>,
}

#[derive(Clone, Default)]
struct MoonGenerator {
    size: f32,
    orbit: Orbit,
}

fn random_vector(rng: &mut StdRng) -> Vec3 {
    loop {
        let axis = Vec3::new(
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
        );

        if axis.length_squared() > 0.0 {
            return axis.normalize();
        }
    }
}

fn random_orbit(
    rng: &mut StdRng,
    major_length_range: Range<f32>,
    period_range: Range<f32>,
) -> Orbit {
    let plane_normal = Vec3::new(
        rng.random_range(-0.1..0.1),
        1.0,
        rng.random_range(-0.1..0.1),
    )
    .normalize();
    let plane_normal = bevy::math::Dir3::new(plane_normal).unwrap();
    let major_direction = loop {
        let direction = random_vector(rng);
        let direction = direction - plane_normal.as_vec3() * plane_normal.dot(direction);
        if direction.length_squared() > 0.0001 {
            break bevy::math::Dir3::new(direction.normalize()).unwrap();
        }
    };
    let major_length = rng.random_range(major_length_range);
    let minor_length = major_length * rng.random_range(0.99..0.99999);

    Orbit::new(
        plane_normal,
        major_direction,
        major_length,
        minor_length,
        rng.random_range(0.0..TAU),
        rng.random_range(period_range),
    )
}

impl WorldGenerator for PlanetGenerator {
    type Generatable = Planet;
    fn new_from_seed(seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let size = rng.random_range(5..15) as f32;
        let moon_count = rng.random_range(1..=3);
        let moons = (0..moon_count)
            .map(|_| MoonGenerator {
                size: rng.random_range(size * 0.15..size * 0.3),
                orbit: random_orbit(&mut rng, size * 1.8..size * 3.2, 2.0..6.0),
            })
            .collect();

        Self {
            size,
            orbit: random_orbit(&mut rng, 300.0..5000.0, 10.0..30.0),
            moons,
        }
    }

    fn generate(self) -> impl Scene {
        let generator = self;
        let mut sectors_positions = Vec::new();
        let mut last_position = Vec3::new(generator.size, 0., 0.);

        for _ in 0..10 {
            sectors_positions.push(last_position);
            last_position = move_in_direction(&last_position, &Vec3::new(0., -1., 0.), 1.);
        }

        let sectors = sectors_positions
            .into_iter()
            .map(|position| {
                bsn! {
                    Mesh3d(asset_value(Sphere::new(0.5)))
                    MeshMaterial3d::<StandardMaterial>(asset_value(Color::srgb(0.75, 0.75, 0.8)))
                    Transform::from_translation(position)
                }
            })
            .collect::<Vec<_>>();

        let moons = generator
            .moons
            .iter()
            .map(|moon| {
                bsn! {
                    Name("Moon")
                    Mesh3d(asset_value(Sphere::new(moon.size)))
                    MeshMaterial3d::<StandardMaterial>(asset_value(Color::srgb(0.55, 0.55, 0.58)))
                    Transform
                    Orbiting {
                        orbit: { moon.orbit.clone() }
                    }
                }
            })
            .collect::<Vec<_>>();

        bsn! {
            Name("Planet")
            Mesh3d(asset_value(Sphere::new(generator.size)))
            MeshMaterial3d::<StandardMaterial>(asset_value(Color::srgb(0.2, 0.5, 1.0)))
            Transform
            FollowCameraTarget::new(generator.size * 2.)
            Orbiting {
                orbit: { generator.orbit.clone() }
            }
            Children [
                {sectors}
                --
                {moons}
                --
                Transform::from_xyz(0., generator.size + 1., 0.)
                Text2d("N")
            ]
            Planet { generator }
        }
    }
}
