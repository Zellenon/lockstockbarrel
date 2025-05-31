use actors::actor_plugin;
use ai::AIPlugin;
use avian2d::{prelude::Gravity, PhysicsPlugins};
use bevy::{
    math::Vec2,
    prelude::{App, Plugin},
};

use bevy_turborand::prelude::RngPlugin;
use camera::CameraPlugin;
use player::player_plugin;
use projectile::projectile_plugin;
use weapons::weapon_plugin;

pub mod actors;
pub mod ai;
pub mod camera;
pub mod events;
pub mod physics;
pub mod player;
pub mod projectile;
pub mod utils;
pub mod weapons;

pub struct TwinStickPlugin;

impl Plugin for TwinStickPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RngPlugin::default(), PhysicsPlugins::default()));
        app.insert_resource(Gravity(Vec2::ZERO));

        actor_plugin(app);
        player_plugin(app);
        projectile_plugin(app);
        weapon_plugin(app);

        app.add_plugins(AIPlugin);

        app.add_plugins(CameraPlugin);
    }
}
