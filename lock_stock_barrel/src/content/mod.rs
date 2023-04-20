use std::sync::Arc;

use bevy::{
    ecs::system::EntityCommands,
    prelude::{Entity, Plugin, Vec2},
};
use bevy_composable::tree::EntityCommandSet;
use bevy_mod_transform2d::transform2d::Transform2d;
use twin_stick::actors::Tracking;

use self::stats::{ensure_health, ensure_speed, sync_health_to_health, sync_speed_to_speed};

pub mod actor_bits;
pub mod enemies;
pub mod stats;
pub mod weapons;

pub struct ContentPlugin;

impl Plugin for ContentPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(ensure_speed);
        app.add_system(ensure_health);
        app.add_system(sync_speed_to_speed);
        app.add_system(sync_health_to_health);
    }
}

pub fn shift_pos(pos: impl Into<Vec2>) -> EntityCommandSet {
    let new_pos = pos.into();
    Arc::new(move |commands: &mut EntityCommands| {
        commands.insert(Transform2d::from_translation(new_pos));
    })
}

pub fn shift_tracking(tracking: Option<Entity>) -> EntityCommandSet {
    Arc::new(move |commands: &mut EntityCommands| {
        commands.insert(Tracking(tracking));
    })
}
