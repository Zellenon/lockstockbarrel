use bevy::prelude::*;

use crate::{enemies::Enemy, player::Player};

pub struct AIPlugin;

#[derive(Component)]
pub struct TrackerAI;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(do_tracker_ai);
    }
}

fn do_tracker_ai(
    player: Query<&Transform, With<Player>>,
    mut ais: Query<(&mut Enemy, &Transform), With<TrackerAI>>,
) {
    let player_pos = player.single().translation.truncate();
    for (mut enemy, transform) in ais.iter_mut() {
        enemy.desired_direction = player_pos - transform.translation.truncate();
    }
}
