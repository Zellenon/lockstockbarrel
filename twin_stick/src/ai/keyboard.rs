use bevy::prelude::{Component, Input, KeyCode, Query, Res, Vec2, With};

use crate::actors::Actor;

#[derive(Component)]
pub struct KeyboardAI;

pub(crate) fn keyboard_input_handler(
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
