use actors::ActorPlugin;
use ai::AIPlugin;
use bevy::app::AppExit;
// use bevy::asset::AssetServerSettings;
use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_ninepatch::NinePatchPlugin;
use bevy_prototype_lyon::prelude as lyon;
use bevy_prototype_lyon::prelude::ShapePlugin;
use bevy_rapier2d::prelude::*;
use enemies::EnemyPlugin;
use player::PlayerPlugin;
use projectile::ProjectilePlugin;
use stats::StatPlugin;
use weapons::WeaponPlugin;

mod actors;
mod ai;
mod enemies;
mod pause;
mod player;
mod projectile;
mod stats;
mod utils;
mod weapons;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    MainMenu,
    Game,
    Pause,
    Exit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameTimerState {
    Playing,
    Paused,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .build()
            .set(WindowPlugin {
                window: WindowDescriptor {
                    title: "Lock Stock and Barrel".to_string(),
                    fit_canvas_to_parent: true,
                    // mode: WindowMode::BorderlessFullscreen,
                    width: 1600.,
                    height: 900.,
                    // monitor: todo!(),
                    // resizable: todo!(),
                    // cursor_visible: todo!(),
                    // cursor_grab_mode: todo!(),
                    ..Default::default()
                },
                ..default()
            })
            .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin),
    )
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.))
    .add_plugin(NinePatchPlugin::<()>::default())
    .add_plugin(ShapePlugin);

    app.add_plugin(PlayerPlugin)
        .add_plugin(StatPlugin)
        .add_plugin(ActorPlugin)
        .add_plugin(WeaponPlugin)
        .add_plugin(ProjectilePlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(AIPlugin);

    app.add_system_set(SystemSet::on_enter(AppState::Exit).with_system(exit));

    if cfg!(debug_assertions) {
        app.add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(WorldInspectorPlugin::new());
    }

    app.insert_resource(ClearColor(Color::rgb(
        0xA9 as f32 / 255.0,
        0xA9 as f32 / 255.0,
        0xAF as f32 / 255.0,
    )));
    // Enable hot reloading
    // .insert_resource(AssetServerSettings {
    //     watch_for_changes: true,
    //     ..default()
    // })

    app.add_startup_system(setup)
        .add_startup_system(spawn_walls);

    app.run();

    Ok(())
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

fn exit(mut app_exit_events: EventWriter<AppExit>) {
    app_exit_events.send(AppExit);
}
