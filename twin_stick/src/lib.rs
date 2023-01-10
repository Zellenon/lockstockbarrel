pub extern crate bevy_rapier2d;
use bevy::prelude::{App, Color, Commands, Plugin, ResMut, Transform, Vec2};
use bevy_prototype_lyon::prelude as lyon;
use bevy_prototype_lyon::prelude::ShapePlugin;
use bevy_rapier2d::prelude::{
    Collider, NoUserData, RapierConfiguration, RapierPhysicsPlugin, RigidBody,
};

use self::{
    actors::ActorPlugin, ai::AIPlugin, enemies::EnemyPlugin, player::PlayerPlugin,
    projectile::ProjectilePlugin, weapons::WeaponPlugin,
};

pub mod actors;
pub mod ai;
pub mod enemies;
pub mod player;
pub mod projectile;
pub mod utils;
pub mod weapons;

pub struct TwinStickPlugin;

impl Plugin for TwinStickPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.))
            .add_plugin(ShapePlugin);

        app.add_plugin(PlayerPlugin)
            .add_plugin(ActorPlugin)
            .add_plugin(WeaponPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(AIPlugin)
            .add_plugin(ProjectilePlugin);
    }
}

fn setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::new(0., 0.);
}

pub fn obstacle_builder(commands: &mut Commands, x: f32, y: f32, width: f32, height: f32) {
    commands
        .spawn(lyon::GeometryBuilder::build_as(
            &lyon::shapes::Rectangle {
                extents: Vec2::new(width, height),
                origin: lyon::shapes::RectangleOrigin::Center,
            },
            lyon::DrawMode::Outlined {
                fill_mode: lyon::FillMode::color(Color::TEAL),
                outline_mode: lyon::StrokeMode::color(Color::TEAL),
            },
            Transform::default(),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(width / 2., height / 2.))
        .insert(Transform::from_xyz(x, y, 0.0));
}
