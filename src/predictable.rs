use bevy::math::Vec3;

pub trait Predictable {
    fn position_at(&self, time: f32) -> Vec3;
    fn cycle_duration(&self) -> f32;
}
