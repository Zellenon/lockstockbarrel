use std::sync::Arc;

use bevy::{
    ecs::system::EntityCommands,
    prelude::{Plugin, Transform},
};
use bevy_composable::EntityCommandSet;

pub mod enemies;

pub fn shift_pos(pos: impl Into<Transform>) -> EntityCommandSet {
    let new_pos = pos.into();
    Arc::new(move |commands: &mut EntityCommands| {
        commands.insert(new_pos);
    })
}
