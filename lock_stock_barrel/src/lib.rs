use bevy::app::AppExit;
// use bevy::asset::AssetServerSettings;
use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_stats::StatPlugin;
use twin_stick::{obstacle_builder, TwinStickPlugin};

mod pause;

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

#[bevy_main]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    );

    app.add_plugin(TwinStickPlugin).add_plugin(StatPlugin);

    app.add_state(AppState::MainMenu);
    app.add_system_set(SystemSet::on_enter(AppState::Exit).with_system(exit));

    if cfg!(debug_assertions) {
        app.add_plugin(WorldInspectorPlugin::new());
        // .add_plugin(RapierDebugRenderPlugin::default());
    }

    app.insert_resource(ClearColor(Color::rgb(
        0xA9 as f32 / 255.0,
        0xA9 as f32 / 255.0,
        0xAF as f32 / 255.0,
    )));
    // Enable hot reloading // Figure out how to fix this I guess
    // .insert_resource(AssetServerSettings {
    //     watch_for_changes: true,
    //     ..default()
    // })

    app.add_startup_system(spawn_walls);

    app.run();

    Ok(())
}

fn spawn_walls(mut commands: Commands) {
    obstacle_builder(&mut commands, -1000., 0., 50., 2000.);
    obstacle_builder(&mut commands, 1000., 0., 50., 2000.);
    obstacle_builder(&mut commands, 0., 1000., 2000., 50.);
    obstacle_builder(&mut commands, 0., -1000., 2000., 50.);
}

fn exit(mut app_exit_events: EventWriter<AppExit>) {
    app_exit_events.send(AppExit);
}
