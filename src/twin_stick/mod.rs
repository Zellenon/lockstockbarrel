use bevy::prelude::{App, Plugin};

use self::{
    actors::ActorPlugin, ai::AIPlugin, enemies::EnemyPlugin, player::PlayerPlugin,
    projectile::ProjectilePlugin, weapons::WeaponPlugin,
};

pub mod actors;
pub mod ai;
pub mod enemies;
pub mod player;
pub mod projectile;
pub mod weapons;

pub struct TwinStickPlugin;

impl Plugin for TwinStickPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_plugin(ActorPlugin)
            .add_plugin(WeaponPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(AIPlugin)
            .add_plugin(ProjectilePlugin);
    }
}
