use bevy::color::Color;
use bevy::ecs::component::Component;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::query;
use bevy::ecs::system::Query;
use bevy::ecs::system::Res;
use bevy::gizmos::gizmos::Gizmos;
use bevy::math::Vec3;
use bevy::time::Time;
use bevy::transform::components::GlobalTransform;
use bevy::transform::components::Transform;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;

pub trait Predictable: Send + Sync + Clone {
    fn predict_position(&self, time: f32) -> Vec3;
    fn cycle_duration(&self) -> f32;
}

pub struct SharedPredictable<T>
where
    T: Clone,
{
    reference: Arc<RwLock<T>>,
}

impl<T> SharedPredictable<T>
where
    T: Clone,
{
    pub fn reader(&self) -> PredictableReader<T> {
        PredictableReader {
            reference: Arc::clone(&self.reference),
        }
    }

    pub fn update(&self, value: T) {
        let mut boxed = self.reference.write().expect("predictable lock poisoned");
        *boxed = value;
    }
}

impl<T> Clone for SharedPredictable<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            reference: Arc::new(RwLock::new(self.reference.read().unwrap().clone())),
        }
    }
}

#[derive(Clone)]
pub struct PredictableReader<T>
where
    T: Clone,
{
    reference: Arc<RwLock<T>>,
}

#[derive(Component, Clone)]
pub struct PredictableComponent {
    shared_predictable: SharedPredictable<()>,
}

pub fn update_predictables(
    time: Res<Time>,
    mut predictable_query: Query<(&PredictableComponent, &mut Transform)>,
) {
    let now = time.elapsed_secs();
    for (predictable, mut transform) in predictable_query.iter_mut() {
        transform.translation = predictable
            .shared_predictable
            .reader()
            .predict_position(now);
    }
}

pub fn draw_predictables(
    time: Res<Time>,
    mut gizmos: Gizmos,
    predictable_query: Query<(&PredictableComponent, Option<&ChildOf>)>,
    global_transform_query: Query<&GlobalTransform>,
) {
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
            let position_a = predictable.predict_position(time_a);
            let position_b = predictable.predict_position(time_b);

            let start = parent_transform.map_or(position_a, |transform| {
                transform.transform_point(position_a)
            });

            let end = parent_transform.map_or(position_b, |transform| {
                transform.transform_point(position_b)
            });

            gizmos.line(position_a, position_b, Color::srgb(0.2, 0.8, 1.0));
        }
    }
}
