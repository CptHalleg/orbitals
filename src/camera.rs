use bevy::{
    app::{Plugin, Update},
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

pub struct OrbitCameraPlugin;
impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, orbit_camera);
    }
}

#[derive(Component, Clone, Default)]
pub struct OrbitCameraTargetable {
    pub radius: f32,
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Component, Clone, Default)]
#[relationship_target(relationship = OrbitCamera)]
pub struct OrbitCameras(Vec<Entity>);

#[derive(Component, Clone)]
#[relationship(relationship_target = OrbitCameras)]
pub struct OrbitCamera(Entity);

fn orbit_camera(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut cameras: Query<(&OrbitCamera, &mut Transform)>,
    mut targetables: Query<(&Transform, &mut OrbitCameraTargetable), Without<OrbitCamera>>,
) {
    let delta = time.delta_secs();
    let orbit_speed = 1.5;

    for (camera, mut transform) in &mut cameras {
        if let Ok((target_transform, mut targetable)) = targetables.get_mut(camera.get()) {
            let mut yaw_direction = 0.;
            let mut pitch_direction = 0.;

            if keys.pressed(KeyCode::ArrowRight) {
                yaw_direction += 1.0;
            }
            if keys.pressed(KeyCode::ArrowLeft) {
                yaw_direction -= 1.0;
            }
            if keys.pressed(KeyCode::ArrowUp) {
                pitch_direction += 1.0;
            }
            if keys.pressed(KeyCode::ArrowDown) {
                pitch_direction -= 1.0;
            }

            targetable.yaw += yaw_direction * orbit_speed * delta;
            targetable.pitch =
                (targetable.pitch + pitch_direction * orbit_speed * delta).clamp(-1.5, 1.5);

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
