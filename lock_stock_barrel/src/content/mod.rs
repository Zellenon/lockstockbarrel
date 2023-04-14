use std::sync::Arc;

use bevy::{
    ecs::system::EntityCommands,
    prelude::{Entity, Transform, Vec2},
};
use bevy_composable::tree::EntityCommandSet;
use twin_stick::actors::Tracking;

pub mod actor_bits;
pub mod enemies;
pub mod weapons;

pub fn shift_pos(pos: impl Into<Vec2>) -> EntityCommandSet {
    let new_pos = pos.into();
    Arc::new(move |commands: &mut EntityCommands| {
        commands.insert(Transform::from_translation(new_pos.extend(0.)));
    })
}

pub fn shift_tracking(tracking: Option<Entity>) -> EntityCommandSet {
    Arc::new(move |commands: &mut EntityCommands| {
        commands.insert(Tracking(tracking));
    })
}
