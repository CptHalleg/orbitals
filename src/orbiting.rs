use crate::predictable::Predictable;
use bevy::math::ops::{cos, sin};
use bevy::prelude::*;
use std::f32::consts::TAU;

pub struct OrbitingPlugin;

impl Plugin for OrbitingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_predictables::<Orbiting>);
        app.add_systems(Update, draw_predictables::<Orbiting>);
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
        self.position_at_angle(new_angle)
    }

    fn position_at_angle(&self, angle: f32) -> Vec3 {
        let focus_distance = (self.major_length.powi(2) - self.minor_length.powi(2))
            .max(0.0)
            .sqrt();
        let focus_offset = self.orientation * Vec3::X * focus_distance;

        self.major_axis() * sin(angle) + self.minor_axis() * cos(angle) - focus_offset
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

impl Predictable for Orbiting {
    fn position_at(&self, time: f32) -> Vec3 {
        self.orbit.position(time)
    }

    fn cycle_duration(&self) -> f32 {
        TAU * self.orbit.period()
    }
}

fn update_predictables<T>(time: Res<Time>, mut predictable_query: Query<(&T, &mut Transform)>)
where
    T: Component + Predictable,
{
    for (predictable, mut transform) in predictable_query.iter_mut() {
        transform.translation = predictable.position_at(time.elapsed_secs());
    }
}

fn draw_predictables<T>(time: Res<Time>, mut gizmos: Gizmos, predictable_query: Query<&T>)
where
    T: Component + Predictable,
{
    for predictable in &predictable_query {
        const SEGMENTS: usize = 128;

        let now = time.elapsed_secs();
        let cycle_duration = predictable.cycle_duration();
        for i in 0..SEGMENTS {
            let time_a = now + ((cycle_duration * (i as f32)) / (SEGMENTS as f32));
            let time_b = now + ((cycle_duration * ((i + 1) as f32)) / (SEGMENTS as f32));

            gizmos.line(
                predictable.position_at(time_a),
                predictable.position_at(time_b),
                Color::srgb(0.2, 0.8, 1.0),
            );
        }
    }
}
