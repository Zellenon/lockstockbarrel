use actors::actor_plugin;
use ai::AIPlugin;
use avian2d::PhysicsPlugins;
use bevy::prelude::{App, AppExtStates, Plugin, ResMut, Startup, Vec2};

use bevy_turborand::prelude::RngPlugin;
use camera::CameraPlugin;
use player::player_plugin;

pub mod actors;
pub mod ai;
pub mod camera;
pub mod player;
// pub mod projectile;
pub mod utils;
// pub mod weapons;

pub struct TwinStickPlugin;

impl Plugin for TwinStickPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RngPlugin::default(), PhysicsPlugins::default()));

        actor_plugin(app);
        player_plugin(app);

        app.add_plugins((
            // WeaponPlugin::<T>::default(),
            AIPlugin,
            // ProjectilePlugin::<T>::default(),
        ));

        app.add_plugins(CameraPlugin);
    }
}
