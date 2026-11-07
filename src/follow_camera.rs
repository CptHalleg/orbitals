use crate::skybox::SkyboxConfig;
use bevy::app::Update;
use bevy::camera;
use bevy::camera::{Camera, Camera3d};
use bevy::ecs::query::With;
use bevy::ecs::system::Commands;
use bevy::light::Skybox;
use bevy::log::info;
use bevy::scene::Scene;
use bevy::scene::bsn;
use bevy::{
    app::{Plugin, PostUpdate},
    ecs::{
        component::Component,
        entity::Entity,
        query::Without,
        relationship::Relationship,
        system::{Query, Res},
    },
    input::{ButtonInput, keyboard::KeyCode},
    math::Vec3,
    time::Time,
    transform::components::Transform,
};

pub struct FollowCameraPlugin;
impl Plugin for FollowCameraPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(PostUpdate, follow_camera);
        app.add_systems(Update, cycle_follow_camera_target);
    }
}

#[derive(Component, Clone, Default)]
pub struct FollowCameraTarget {
    priority: u64,

    radius: f32,
    yaw: f32,
    pitch: f32,
}

impl FollowCameraTarget {
    pub fn new(radius: f32) -> Self {
        Self {
            priority: rand::random(),

            radius,
            yaw: 0.,
            pitch: 0.,
        }
    }
}

#[derive(Component, Clone, Default)]
#[relationship_target(relationship = FollowCamera)]
pub struct FollowCameras(Vec<Entity>);

#[derive(Component, Clone)]
#[relationship(relationship_target = FollowCameras)]
pub struct FollowCamera(Entity);

fn follow_camera(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut cameras: Query<(&FollowCamera, &mut Transform)>,
    mut targetables: Query<(&Transform, &mut FollowCameraTarget), Without<FollowCamera>>,
) {
    let delta = time.delta_secs();
    let rotation_speed = 1.5;
    let zoom_speed = 1.;

    for (camera, mut transform) in &mut cameras {
        if let Ok((target_transform, mut targetable)) = targetables.get_mut(camera.get()) {
            let mut yaw_direction = 0.;
            let mut pitch_direction = 0.;
            let mut zoom_direction = 0.;

            if keys.pressed(KeyCode::ArrowRight) {
                yaw_direction += 1.;
            }
            if keys.pressed(KeyCode::ArrowLeft) {
                yaw_direction -= 1.;
            }
            if keys.pressed(KeyCode::ArrowUp) {
                pitch_direction += 1.0;
            }
            if keys.pressed(KeyCode::ArrowDown) {
                pitch_direction -= 1.0;
            }
            if keys.pressed(KeyCode::Period) {
                zoom_direction += 1.0;
            }
            if keys.pressed(KeyCode::Comma) {
                zoom_direction -= 1.0;
            }

            targetable.yaw += yaw_direction * rotation_speed * delta;
            targetable.pitch =
                (targetable.pitch + pitch_direction * rotation_speed * delta).clamp(-1.5, 1.5);

            targetable.radius += zoom_direction * zoom_speed * delta * (targetable.radius);

            let horizontal_radius = targetable.radius * targetable.pitch.cos();
            let target_position = target_transform.translation;
            transform.translation = target_position
                + Vec3::new(
                    horizontal_radius * targetable.yaw.sin(),
                    targetable.radius * targetable.pitch.sin(),
                    horizontal_radius * targetable.yaw.cos(),
                );

            transform.look_at(target_position, Vec3::Y);
        }
    }
}

fn cycle_follow_camera_target(
    keys: Res<ButtonInput<KeyCode>>,
    target_query: Query<(Entity, &FollowCameraTarget), Without<FollowCamera>>,
    camera_query: Query<(Entity, &FollowCamera), Without<FollowCameraTarget>>,
    mut commands: Commands,
) {
    if !keys.just_pressed(KeyCode::Tab) {
        return;
    }

    let mut targets = target_query
        .iter()
        .collect::<Vec<(Entity, &FollowCameraTarget)>>();

    targets.sort_by_key(|(_, target)| target.priority);

    if targets.is_empty() {
        return;
    }

    for (camera_entity, camera) in &camera_query {
        let Ok((_, current_target)) = target_query.get(camera.get()) else {
            info!("target not found");
            commands
                .entity(camera_entity)
                .insert(FollowCamera(targets[0].0));
            continue;
        };

        let current_key = (current_target.priority, camera.get());

        let Ok(index) = targets
            .binary_search_by_key(&current_key, |(entity, target)| (target.priority, *entity))
        else {
            info!("current key not found");
            continue;
        };

        let next_target = targets[(index + 1) % targets.len()].0;
        commands
            .entity(camera_entity)
            .insert(FollowCamera(next_target));
    }
}
