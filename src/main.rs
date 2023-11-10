use bevy::{
    app::App,
    prelude::{default, ClearColor, PluginGroup},
    render::color::Color,
    window::{Window, WindowPlugin},
    DefaultPlugins,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_stats::StatPlugin;
use bevy_twin_stick::TwinStickPlugin;
use game::GamePlugin;
use states::StatePlugin;

mod content;
mod game;
mod hud;
mod mainmenu;
mod pause;
mod states;

// #[bevy_main]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.build().set(WindowPlugin {
        primary_window: Some(Window {
            title: "Lock Stock and Barrel".to_string(),
            fit_canvas_to_parent: true,
            // mode: WindowMode::BorderlessFullscreen,
            // width: 1600.,
            // height: 900.,
            // monitor: todo!(),
            // resizable: todo!(),
            // cursor_visible: todo!(),
            // cursor_grab_mode: todo!(),
            ..Default::default()
        }),
        ..default()
    }));

    app.add_plugins((TwinStickPlugin, StatPlugin));

    app.add_plugins((StatePlugin, GamePlugin));

    if cfg!(debug_assertions) {
        app.add_plugins(WorldInspectorPlugin::new());
        // app.add_plugin(RapierDebugRenderPlugin::default());
    }

    app.insert_resource(ClearColor(Color::rgb(
        0xA9 as f32 / 255.0,
        0xA9 as f32 / 255.0,
        0xAF as f32 / 255.0,
    )));
    // Enable hot reloading // Figure out how to fix this I guess
    // app.insert_resource(AssetServerSettings {
    //     watch_for_changes: true,
    //     ..default()
    // });

    app.run();

    Ok(())
}
