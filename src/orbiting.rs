use bevy::math::ops::{cos, sin};
use bevy::prelude::*;

pub struct OrbitingPlugin;

impl Plugin for OrbitingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_orbits);
    }
}

#[derive(Component, Clone, Default)]
pub struct Orbiting {
    pub first_axis: Vec3,
    pub second_axis: Vec3,
    pub phase: f32,
    pub period: f32,
}

fn update_orbits(time: Res<Time>, mut orbiting_query: Query<(&Orbiting, &mut Transform)>) {
    for (orbiting, mut transform) in orbiting_query.iter_mut() {
        let new_angle = orbiting.phase + (time.elapsed_secs() / orbiting.period);
        let new_position =
            orbiting.first_axis * sin(new_angle) + orbiting.second_axis * cos(new_angle);

        transform.translation = new_position;
    }
}
