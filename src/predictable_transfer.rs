use crate::predictable::Predictable;
use std::sync::Arc;

struct PredictableTransfer {
    from: Arc<dyn Predictable>,
    to: Arc<dyn Predictable>,
    start_time: f32,
    finish_time: f32,
}

impl Predictable for PredictableTransfer {
    fn predict_position(&self, time: f32) -> bevy::math::Vec3 {
        todo!()
    }

    fn cycle_duration(&self) -> f32 {
        todo!()
    }

    fn clone_box(&self) -> Box<dyn Predictable> {
        todo!()
    }
}
