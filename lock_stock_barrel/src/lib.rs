use bevy::{
    app::App,
    prelude::{bevy_main, default, ClearColor, PluginGroup},
    render::color::Color,
    window::{WindowDescriptor, WindowPlugin},
    DefaultPlugins,
};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_stats::StatPlugin;
use game::GamePlugin;
use prettegui::GUIPlugin;
use states::StatePlugin;
use twin_stick::TwinStickPlugin;

mod content;
mod game;
mod hud;
mod mainmenu;
mod pause;
mod states;

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

    app.add_plugin(TwinStickPlugin)
        .add_plugin(StatPlugin)
        .add_plugin(GUIPlugin);

    app.add_plugin(StatePlugin);
    app.add_plugin(GamePlugin);

    if cfg!(debug_assertions) {
        app.add_plugin(WorldInspectorPlugin::new());
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
