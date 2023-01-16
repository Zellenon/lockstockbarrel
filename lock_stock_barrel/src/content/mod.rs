use std::sync::Arc;

use bevy::{
    ecs::system::EntityCommands,
    prelude::{Transform, Vec2},
};
use bevy_composable::EntityCommandSet;

pub mod enemies;

pub fn shift_pos(pos: impl Into<Vec2>) -> EntityCommandSet {
    let new_pos = pos.into();
    Arc::new(move |commands: &mut EntityCommands| {
        commands.insert(Transform::from_translation(new_pos.extend(0.)));
    })
}
