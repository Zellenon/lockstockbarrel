use bevy::{
    app::App,
    prelude::{default, ClearColor, PluginGroup},
    render::color::Color,
    window::{Window, WindowPlugin},
    DefaultPlugins,
};
use bevy_egui::EguiPlugin;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_stats::StatPlugin;
use game::GamePlugin;
use states::StatePlugin;
use twin_stick::TwinStickPlugin;

mod content;
mod game;
mod hud;
mod mainmenu;
mod pause;
mod states;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .build()
            .set(WindowPlugin {
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
            })
            .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin),
    );

    app.add_plugins(EguiPlugin);

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
