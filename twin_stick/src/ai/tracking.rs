use bevy::prelude::{Component, Query, With};
use bevy_mod_transform2d::transform2d::Transform2d;

use crate::{actors::Actor, player::Player};

#[derive(Component)]
pub struct TrackerAI {
    pub precision: f32,
}

pub(crate) fn do_tracker_ai(
    player: Query<&Transform2d, With<Player>>,
    mut ais: Query<(&mut Actor, &Transform2d, &TrackerAI)>,
) {
    let player_pos = player.single().translation;
    for (mut enemy, transform, tracker) in ais.iter_mut() {
        enemy.desired_direction +=
            tracker.precision * (player_pos - transform.translation).clamp_length_max(1.);
    }
}
