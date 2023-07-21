pub extern crate bevy_mod_transform2d;
pub extern crate bevy_rapier2d;
pub extern crate bevy_turborand;
use bevy::prelude::{App, Plugin, ResMut, Startup, Vec2};
use bevy_mod_transform2d::{transform2d::Transform2d, Transform2dPlugin};

use bevy_prototype_lyon::prelude::ShapePlugin;
use bevy_rapier2d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};
use bevy_turborand::prelude::RngPlugin;
use stats::{Health, Knockback, Speed};

use self::{
    actors::ActorPlugin, ai::AIPlugin, player::PlayerPlugin, projectile::ProjectilePlugin,
    weapons::WeaponPlugin,
};

pub mod actors;
pub mod ai;
pub mod player;
pub mod projectile;
pub mod stats;
pub mod transform2d_mods;
pub mod utils;
pub mod weapons;

pub struct TwinStickPlugin;

impl Plugin for TwinStickPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.),
            Transform2dPlugin,
            ShapePlugin,
            RngPlugin::default(),
        ));

        app.add_plugins((
            PlayerPlugin,
            ActorPlugin,
            WeaponPlugin,
            AIPlugin,
            ProjectilePlugin,
        ));

        app.register_type::<Transform2d>();
        app.register_type::<Speed>();
        app.register_type::<Health>();
        app.register_type::<Knockback>();

        app.add_systems(Startup, rapier_config_setup);
    }
}

fn rapier_config_setup(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::new(0., 0.);
}
