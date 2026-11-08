use crate::follow_camera::FollowCameraPlugin;
use crate::follow_camera::FollowCameras;
use crate::follow_camera::camera_entity;
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
        .add_plugins(OrbitingPlugin)
        .add_plugins(FollowCameraPlugin)
        .add_plugins(SkyboxPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, inspector_setup)
        .run();
}

fn setup(mut commands: Commands) {
    let mut rng = rand::rng();
    commands.spawn_scene(SolarSystemGenerator::new_from_seed(rng.next_u64()).generate());
}

fn inspector_setup(mut egui_settings: ResMut<EguiGlobalSettings>) {
    egui_settings
        .input_system_settings
        .run_write_keyboard_input_messages_system = false;
}
