use bevy::light::Skybox;
use bevy::math::ops::{cos, sin};
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
        .add_systems(Update, orbiting)
        .run();
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    let skybox_handle = asset_server.load("cubemap_stacked.png");
    let mut rng = rand::rng();
    let scene = bsn_list! {
        #Sun
        MeshMaterial3d::<StandardMaterial>(asset_value(Color::srgb(1., 0.1, 0.1)))
        Mesh3d(asset_value(Sphere::new(50.)))
        --
        #Earth
        @planet(rng.random())
        Orbiting {
            first_axis: Vec3::new(300., 50., 0.),
            second_axis: Vec3::new(0., 0., 200.),

            angle: 0.,
            speed: 1.,
        }
        Transform{translation: Vec3::new(100., 0. , 0.)}
        OrbitCameras [
            Transform
            Skybox {
                image: Option::Some(skybox_handle),
                brightness: 1000.0,
            }
            Camera3d
            Camera{order:0}
        ]
        ChildOf(#Sun)
        --
        DirectionalLight {
            illuminance: 10_000.0,
        }
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4))
    };
    commands.spawn_scene_list(scene);
}

#[derive(Component, Clone, Default)]
pub struct Orbiting {
    pub first_axis: Vec3,
    pub second_axis: Vec3,

    pub angle: f32,
    pub speed: f32,
}

fn orbiting(time: Res<Time>, mut orbiting_query: Query<(&mut Orbiting, &mut Transform)>) {
    for (mut orbiting, mut transform) in orbiting_query.iter_mut() {
        let new_angle = orbiting.angle + orbiting.speed * time.delta_secs();

        let new_position =
            orbiting.first_axis * sin(new_angle) + orbiting.second_axis * cos(new_angle);

        transform.translation = new_position;
        orbiting.angle = new_angle;
    }
}
