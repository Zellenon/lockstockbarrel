use actors::ActorPlugin;
use ai::AIPlugin;
// use bevy::asset::AssetServerSettings;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude as lyon;
use bevy_prototype_lyon::prelude::ShapePlugin;
use bevy_rapier2d::prelude::*;
use enemies::EnemyPlugin;
use player::PlayerPlugin;
use projectile::ProjectilePlugin;
use weapons::WeaponPlugin;

mod actors;
mod ai;
mod enemies;
mod player;
mod projectile;
mod stats;
mod utils;
mod weapons;

fn main() {
    // let mut window_desc = WindowDescriptor::default();
    // window_desc.width = 1600.0;
    // window_desc.height = 900.0;
    // window_desc.title = "Bevy Rider".to_string();
    App::new()
        // .insert_resource(window_desc)
        // Enable hot reloading
        // .insert_resource(AssetServerSettings {
        //     watch_for_changes: true,
        //     ..default()
        // })
        .add_startup_system(setup)
        .add_startup_system(spawn_walls)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.))
        .add_plugin(PlayerPlugin)
        .add_plugin(ActorPlugin)
        .add_plugin(WeaponPlugin)
        .add_plugin(ProjectilePlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(AIPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}

fn setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::new(0., 0.);
}

fn spawn_walls(mut commands: Commands) {
    let mut build_wall = |x, y, width, height| {
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
    };
    build_wall(-1000., 0., 50., 2000.);
    build_wall(1000., 0., 50., 2000.);
    build_wall(0., 1000., 2000., 50.);
    build_wall(0., -1000., 2000., 50.);
}
