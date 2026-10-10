use bevy::light::Skybox;
use bevy::prelude::*;
use rand::RngExt;
use world_generation::planet;

use crate::camera::{OrbitCameraPlugin, OrbitCameras};
use crate::skybox::SkyboxPlugin;

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
        .run();
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn_scene_list(scene(&asset_server));
}

fn scene(asset_server: &AssetServer) -> impl SceneList {
    let skybox_handle = asset_server.load("cubemap_stacked.png");
    let mut rng = rand::rng();
    bsn_list! {
        @planet(rng.random())
        OrbitCameras [
            Transform
            Skybox {
                image: Option::Some(skybox_handle),
                brightness: 1000.0,
            }
            Camera3d
            Camera{order:0}
        ]
        --
        DirectionalLight {
            illuminance: 10_000.0,
        }
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4))
    }
}
