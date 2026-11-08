use bevy::color::Color;
use bevy::ecs::component::Component;
use bevy::ecs::system::Query;
use bevy::ecs::system::Res;
use bevy::gizmos::gizmos::Gizmos;
use bevy::math::Vec3;
use bevy::time::Time;
use bevy::transform::components::Transform;

pub trait Predictable {
    fn position_at(&self, time: f32) -> Vec3;
    fn cycle_duration(&self) -> f32;
}

pub fn update_predictables<T>(time: Res<Time>, mut predictable_query: Query<(&T, &mut Transform)>)
where
    T: Component + Predictable,
{
    for (predictable, mut transform) in predictable_query.iter_mut() {
        transform.translation = predictable.position_at(time.elapsed_secs());
    }
}

pub fn draw_predictables<T>(time: Res<Time>, mut gizmos: Gizmos, predictable_query: Query<&T>)
where
    T: Component + Predictable,
{
    const SEGMENTS_PER_SECOND: f32 = 2.;
    for predictable in &predictable_query {
        let now = time.elapsed_secs();
        let cycle_duration = predictable.cycle_duration();
        let segments = (cycle_duration * SEGMENTS_PER_SECOND).ceil() as usize;
        for i in 0..segments {
            let time_a = now + ((cycle_duration * (i as f32)) / (segments as f32));
            let time_b = now + ((cycle_duration * ((i + 1) as f32)) / (segments as f32));

            gizmos.line(
                predictable.position_at(time_a),
                predictable.position_at(time_b),
                Color::srgb(0.2, 0.8, 1.0),
            );
        }
    }
}
