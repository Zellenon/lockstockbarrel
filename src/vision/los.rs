use bevy::{
    ecs::{entity::Entity, query::Changed, system::Query},
    transform::components::Transform,
};

use super::{eyes::Eye, VisionObjects, LOS};

pub fn update_los(mut seers: Query<(&mut LOS, &Eye), Changed<Eye>>) {
    for (mut seer, eye) in seers.iter_mut() {
        seer.0 = eye.los.clone().into_iter().collect();
    }
}
