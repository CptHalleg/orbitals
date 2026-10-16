use bevy::light::Skybox;
use bevy::math::ops::{cos, sin};
use bevy::prelude::*;
use rand::RngExt;
use world_generation::planet;

use crate::camera::OrbitCameraTargetable;
use crate::camera::{OrbitCameraPlugin, OrbitCameras};
use crate::skybox::SkyboxPlugin;
use std::f32::consts::PI;

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
        OrbitCameraTargetable{radius: 40.}
        OrbitCameras [
            Transform
            Skybox {
                image: Option::Some(skybox_handle),
                brightness: 1000.0,
            }
            Camera3d
            Camera{order:0}
        ]
        PointLight {
            color: Color::srgb(1.,1.,0.1),
            intensity: 1_000_000_000.,
            range: 10_000.,
        }
        --
        #Earth
        @planet(rng.random())
        Orbiting {
            first_axis: Vec3::new(500., 50., 0.),
            second_axis: Vec3::new(0., 0., 200.),

            phase: 0.,
            period: (2. * PI),
        }
        Transform{translation: Vec3::new(100., 0. , 0.)}
        ChildOf(#Sun)
    };
    commands.spawn_scene_list(scene);
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
