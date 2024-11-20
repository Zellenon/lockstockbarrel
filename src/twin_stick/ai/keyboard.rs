use bevy::prelude::Reflect;
use bevy::{
    input::ButtonInput,
    prelude::{Component, KeyCode, Query, Res, Vec2, With},
};

use super::super::actors::Actor;

#[derive(Component, Clone, Copy, PartialEq, Eq, Reflect, Debug)]
pub struct KeyboardAI;

pub(crate) fn keyboard_input_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ais: Query<&mut Actor, With<KeyboardAI>>,
) {
    let mut total_force = Vec2::new(0., 0.);
    if keyboard_input.pressed(KeyCode::KeyA) {
        total_force.x += -1.;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        total_force.x += 1.;
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        total_force.y += 1.;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        total_force.y += -1.;
    }
    for mut actor in ais.iter_mut() {
        actor.desired_direction = total_force;
    }
}
