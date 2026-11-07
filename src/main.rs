use crate::follow_camera::FollowCameraPlugin;
use crate::follow_camera::FollowCameras;
use crate::orbiting::OrbitingPlugin;
use crate::skybox::SkyboxConfig;
use crate::skybox::SkyboxPlugin;
use crate::world_generation::WorldGenerator;
use crate::world_generation::solar_system::SolarSystemGenerator;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiGlobalSettings;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use rand::Rng;

mod follow_camera;
mod orbiting;
mod skybox;
mod sphere_math;
mod world_generation;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(FollowCameraPlugin)
        .add_plugins(OrbitingPlugin)
        .add_plugins(SkyboxPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, inspector_setup)
        .run();
}

pub fn camera_entity() -> impl Scene {
    bsn! {
        FollowCameras [
            Transform
            SkyboxConfig {
                image: "cubemap_stacked.png",
                brightness: 1000.0,
            }
            Camera3d
            Camera{order:0}
        ]
    }
}

fn setup(mut commands: Commands) {
    let mut rng = rand::rng();

    commands.spawn_scene(camera_entity());
    commands.spawn_scene(SolarSystemGenerator::new_from_seed(rng.next_u64()).generate());
}

fn inspector_setup(mut egui_settings: ResMut<EguiGlobalSettings>) {
    egui_settings
        .input_system_settings
        .run_write_keyboard_input_messages_system = false;
}
