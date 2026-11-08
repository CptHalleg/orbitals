use bevy::color::Color;
use bevy::ecs::component::Component;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::system::Query;
use bevy::ecs::system::Res;
use bevy::gizmos::gizmos::Gizmos;
use bevy::math::Vec3;
use bevy::time::Time;
use bevy::transform::components::GlobalTransform;
use bevy::transform::components::Transform;

pub trait Predictable {
    fn position_at(&self, time: f32) -> Vec3;
    fn cycle_duration(&self) -> f32;
}

pub fn update_predictables<T>(time: Res<Time>, mut predictable_query: Query<(&T, &mut Transform)>)
where
    T: Component + Predictable,
{
    let now = time.elapsed_secs();
    for (predictable, mut transform) in predictable_query.iter_mut() {
        transform.translation = predictable.position_at(now);
    }
}

pub fn draw_predictables<T>(
    time: Res<Time>,
    mut gizmos: Gizmos,
    predictable_query: Query<(&T, Option<&ChildOf>)>,
    global_transform_query: Query<&GlobalTransform>,
) where
    T: Component + Predictable,
{
    const SEGMENTS_PER_SECOND: f32 = 2.;
    let now = time.elapsed_secs();
    for (predictable, parent) in &predictable_query {
        let cycle_duration = predictable.cycle_duration();
        let segments = (cycle_duration * SEGMENTS_PER_SECOND).ceil() as usize;
        let parent_transform =
            parent.and_then(|parent| global_transform_query.get(parent.parent()).ok());
        for i in 0..segments {
            let time_a = now + ((cycle_duration * (i as f32)) / (segments as f32));
            let time_b = now + ((cycle_duration * ((i + 1) as f32)) / (segments as f32));
            let position_a = predictable.position_at(time_a);
            let position_b = predictable.position_at(time_b);

            let start = parent_transform.map_or(position_a, |transform| {
                transform.transform_point(position_a)
            });

            let end = parent_transform.map_or(position_b, |transform| {
                transform.transform_point(position_b)
            });

            gizmos.line(start, end, Color::srgb(0.2, 0.8, 1.0));
        }
    }
}
