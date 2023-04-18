pub extern crate bevy_mod_transform2d;
pub extern crate bevy_rapier2d;
pub extern crate bevy_turborand;
use bevy::prelude::{App, Plugin, ResMut, Vec2};
use bevy_mod_transform2d::{transform2d::Transform2d, Transform2dPlugin};

use bevy_prototype_lyon::prelude::ShapePlugin;
use bevy_rapier2d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};
use bevy_turborand::RngPlugin;

use self::{
    actors::ActorPlugin, ai::AIPlugin, player::PlayerPlugin, projectile::ProjectilePlugin,
    weapons::WeaponPlugin,
};

pub mod actors;
pub mod ai;
pub mod player;
pub mod projectile;
pub mod transform2d_mods;
pub mod utils;
pub mod weapons;

pub struct TwinStickPlugin;

impl Plugin for TwinStickPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.))
            .add_plugin(Transform2dPlugin)
            .add_plugin(ShapePlugin);

        app.add_plugin(PlayerPlugin)
            .add_plugin(RngPlugin::default())
            .add_plugin(ActorPlugin)
            .add_plugin(WeaponPlugin)
            .add_plugin(AIPlugin)
            .add_plugin(ProjectilePlugin);

        app.register_type::<Transform2d>();
    }
}

fn setup(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::new(0., 0.);
}
