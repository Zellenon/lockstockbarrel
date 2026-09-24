use bevy::ecs::{query::Changed, system::Query};

use super::{eyes::Eye, LOS};

pub fn update_los(mut seers: Query<(&mut LOS, &Eye), Changed<Eye>>) {
    for (mut seer, eye) in seers.iter_mut() {
        seer.0 = eye.los.clone().into_iter().collect();
    }
}
