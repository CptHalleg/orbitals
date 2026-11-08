use bevy::math::ops::{cos, sin};
use bevy::prelude::*;
use std::f32::consts::TAU;

pub struct OrbitingPlugin;

impl Plugin for OrbitingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_orbits);
        app.add_systems(Update, (update_orbits, draw_orbits));
    }
}

#[derive(Clone)]
pub struct Orbit {
    orientation: Quat,
    major_length: f32,
    minor_length: f32,
    phase: f32,
    period: f32,
}

impl Orbit {
    pub fn new(
        plane_normal: Dir3,
        major_direction: Dir3,
        major_length: f32,
        minor_length: f32,
        phase: f32,
        period: f32,
    ) -> Self {
        let normal = plane_normal.as_vec3();
        let major = major_direction.as_vec3();
        let minor = normal.cross(major).normalize();
        let orientation = Quat::from_mat3(&Mat3::from_cols(major, minor, normal)).normalize();
        Self {
            orientation,
            major_length,
            minor_length,
            phase,
            period,
        }
    }

    pub fn major_axis(&self) -> Vec3 {
        self.orientation * Vec3::X * self.major_length
    }

    pub fn minor_axis(&self) -> Vec3 {
        self.orientation * Vec3::Y * self.minor_length
    }

    pub fn phase(&self) -> f32 {
        self.phase
    }

    pub fn period(&self) -> f32 {
        self.period
    }

    pub fn position(&self, time: f32) -> Vec3 {
        let new_angle = self.phase + (time / self.period);
        self.major_axis() * sin(new_angle) + self.minor_axis() * cos(new_angle)
    }
}

impl Default for Orbit {
    fn default() -> Self {
        Self::new(Dir3::Y, Dir3::X, 10., 5., 0., 5.)
    }
}

#[derive(Component, Clone, Default)]
pub struct Orbiting {
    pub orbit: Orbit,
}

fn update_orbits(time: Res<Time>, mut orbiting_query: Query<(&Orbiting, &mut Transform)>) {
    for (orbiting, mut transform) in orbiting_query.iter_mut() {
        transform.translation = orbiting.orbit.position(time.elapsed_secs());
    }
}

fn draw_orbits(mut gizmos: Gizmos, orbiting_query: Query<&Orbiting>) {
    const SEGMENTS: usize = 96;

    for orbiting in &orbiting_query {
        let orbit = &orbiting.orbit;

        for segment in 0..SEGMENTS {
            let angle_a = TAU * segment as f32 / SEGMENTS as f32;
            let angle_b = TAU * (segment + 1) as f32 / SEGMENTS as f32;

            let point_a = orbit.major_axis() * sin(angle_a) + orbit.minor_axis() * cos(angle_a);
            let point_b = orbit.major_axis() * sin(angle_b) + orbit.minor_axis() * cos(angle_b);

            gizmos.line(point_a, point_b, Color::srgb(0.2, 0.8, 1.0));
        }
    }
}
