use bevy::{
    math::Vec3Swizzles,
    prelude::{Component, Query, Reflect, Transform, With},
};

use crate::twin_stick::{actors::Actor, player::Player};

#[derive(Component, Clone, Copy, PartialEq, Reflect, Debug)]
pub struct TrackerAI {
    pub precision: f32,
}

pub(crate) fn do_tracker_ai(
    player: Query<&Transform, With<Player>>,
    mut ais: Query<(&mut Actor, &Transform, &TrackerAI)>,
) {
    let player_pos = player.single().translation;
    for (mut enemy, transform, tracker) in ais.iter_mut() {
        enemy.desired_direction += tracker.precision
            * (player_pos - transform.translation)
                .clamp_length_max(1.)
                .xy();
    }
}
