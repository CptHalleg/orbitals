use bevy::prelude::*;
use rand::RngExt;
use rand::{SeedableRng, rngs::StdRng};

#[derive(Component, Clone, Default)]
struct Planet;

use crate::camera::OrbitCameraTargetable;
use crate::sphere_math::move_in_direction;

pub fn planet(seed: u64) -> impl Scene {
    let mut rng = StdRng::seed_from_u64(seed);
    let radius = rng.random_range(10.0..15.0);

    let mut sectors_positions = Vec::new();
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
        OrbitCameraTargetable{radius: 40.}
        Children [
            {sectors}
            --
            Transform::from_xyz(0., radius +1., 0.)
            Text2d("N")
        ]
    }
}
