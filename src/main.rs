use bevy::prelude::Color;
use bevy::{
    app::App,
    prelude::{default, ClearColor, PluginGroup},
    window::{Window, WindowPlugin},
    DefaultPlugins,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_stats::StatPlugin;
use bevy_twin_stick::TwinStickPlugin;
use debug::DebugPlugin;
use game::GamePlugin;
use states::StatePlugin;
use ui::UiPlugin;

mod content;
mod debug;
mod game;
mod graphics;
mod hud;
mod mainmenu;
mod pause;
mod states;
mod ui;

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

    app.add_plugins((DebugPlugin, StatePlugin, GamePlugin, UiPlugin));

    app.insert_resource(ClearColor(Color::srgb(0.7, 0.7, 0.7)));
    app.run();

    Ok(())
}
