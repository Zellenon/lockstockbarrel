use std::sync::Arc;

use bevy::{
    ecs::system::EntityCommands,
    prelude::{Entity, IntoSystemConfigs, Plugin, Update, Vec2},
};
use bevy_composable::tree::EntityCommandSet;
use bevy_mod_transform2d::transform2d::Transform2d;
use bevy_stats::systems::{delete_stat_mod, StatRegisterable};
use twin_stick::actors::Tracking;

use self::{
    projectile_components::{apply_slow_on_hit, damaging_projectile, tick_fading_slow},
    stats::{
        ensure_health, ensure_speed, sync_health_to_health, sync_speed_to_speed, Damage, Health,
        Speed,
    },
};

pub mod actor_bits;
pub mod enemies;
pub mod projectile;
pub mod projectile_components;
pub mod stats;
pub mod weapons;

pub struct ContentPlugin;

impl Plugin for ContentPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_stat::<Speed>()
            .register_resource::<Health>()
            .register_stat::<Damage>();

        app.add_systems(
            Update,
            (
                ensure_speed,
                ensure_health,
                sync_speed_to_speed,
                sync_health_to_health,
                tick_fading_slow.before(delete_stat_mod),
                damaging_projectile,
                apply_slow_on_hit,
            ),
        );
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
