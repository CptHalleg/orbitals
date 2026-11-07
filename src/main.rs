use bevy::light::Skybox;
use bevy::math::ops::{cos, sin};
use bevy::prelude::*;
use rand::RngExt;
use world_generation::planet;

use crate::camera::{OrbitCameraPlugin, OrbitCameras};
use crate::skybox::SkyboxPlugin;
use crate::world_generation::WorldGeneratable;
use crate::world_generation::solar_system::SolarSystem;
use crate::world_generation::solar_system::SolarSystemGenerator;
use crate::world_generation::sun;

mod camera;
mod skybox;
mod sphere_math;
mod world_generation;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(OrbitCameraPlugin)
        .add_plugins(SkyboxPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, orbiting)
        .run();
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    let skybox_handle = asset_server.load("cubemap_stacked.png");
    let mut rng = rand::rng();
    let scene = bsn! {
        OrbitCameras [
            Transform
            Skybox {
                image: Option::Some(skybox_handle),
                brightness: 1000.0,
            }
            Camera3d
            Camera{order:0}
        ]
    };
    commands.spawn_scene(scene);
    commands.spawn_scene(SolarSystemGenerator::new_from_seed(0).generate());
}

#[derive(Component, Clone, Default)]
pub struct Orbiting {
    pub first_axis: Vec3,
    pub second_axis: Vec3,

    pub phase: f32,
    pub period: f32,
}

fn orbiting(time: Res<Time>, mut orbiting_query: Query<(&Orbiting, &mut Transform)>) {
    for (orbiting, mut transform) in orbiting_query.iter_mut() {
        let new_angle = orbiting.phase + (time.elapsed_secs() / orbiting.period);

        let new_position =
            orbiting.first_axis * sin(new_angle) + orbiting.second_axis * cos(new_angle);

        transform.translation = new_position;
    }
}
