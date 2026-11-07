use crate::follow_camera::{FollowCameraPlugin, FollowCameras};
use crate::orbiting::OrbitingPlugin;
use crate::skybox::SkyboxPlugin;
use crate::world_generation::WorldGenerator;
use crate::world_generation::solar_system::SolarSystemGenerator;
use bevy::light::Skybox;
use bevy::prelude::*;
use rand::Rng;

mod follow_camera;
mod orbiting;
mod skybox;
mod sphere_math;
mod world_generation;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FollowCameraPlugin)
        .add_plugins(OrbitingPlugin)
        .add_plugins(SkyboxPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    let skybox_handle = asset_server.load("cubemap_stacked.png");
    let mut rng = rand::rng();
    let scene = bsn! {
        FollowCameras [
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
    commands.spawn_scene(SolarSystemGenerator::new_from_seed(rng.next_u64()).generate());
}
