use bevy::prelude::*;

use crate::{
    actors::Actor,
    player::{player_exists, Player},
};

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(do_tracker_ai.run_if(player_exists))
            .add_system(keyboard_input_handler.run_if(player_exists));
    }
}

#[derive(Component)]
pub struct TrackerAI;

fn do_tracker_ai(
    player: Query<&Transform, With<Player>>,
    mut ais: Query<(&mut Actor, &Transform), With<TrackerAI>>,
) {
    let player_pos = player.single().translation.truncate();
    for (mut enemy, transform) in ais.iter_mut() {
        enemy.desired_direction = player_pos - transform.translation.truncate();
    }
}

#[derive(Component)]
pub struct KeyboardAI;

fn keyboard_input_handler(
    keyboard_input: Res<Input<KeyCode>>,
    mut ais: Query<&mut Actor, With<KeyboardAI>>,
) {
    let mut total_force = Vec2::new(0., 0.);
    if keyboard_input.pressed(KeyCode::A) {
        total_force.x += -1.;
    }
    if keyboard_input.pressed(KeyCode::D) {
        total_force.x += 1.;
    }
    if keyboard_input.pressed(KeyCode::W) {
        total_force.y += 1.;
    }
    if keyboard_input.pressed(KeyCode::S) {
        total_force.y += -1.;
    }
    for mut actor in ais.iter_mut() {
        actor.desired_direction = total_force;
    }
}
