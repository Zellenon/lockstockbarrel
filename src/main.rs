#![feature(trivial_bounds)]

use action_system::ActionSystemPlugin;
use assets::AssetPlugin;
use bevy::{
    app::App,
    prelude::{default, ClearColor, Color, PluginGroup},
    window::{Window, WindowPlugin},
    DefaultPlugins,
};
use debug::DebugPlugin;
use game::GamePlugin;
use states::StatePlugin;
use twin_stick::TwinStickPlugin;
use util::UtilPlugin;

mod action_system;
mod arena;
mod assets;
mod content;
mod debug;
mod game;
mod graphics;
mod states;
mod transform2d;
mod twin_stick;
mod ui;
mod util;

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

    app.add_plugins(AssetPlugin);
    // app.add_plugins(StatPlugin);
    app.add_plugins(TwinStickPlugin);
    app.add_plugins(ActionSystemPlugin);

    // app.add_plugins((StatePlugin, UiPlugin));
    app.add_plugins(StatePlugin);
    app.add_plugins(GamePlugin);
    app.add_plugins(DebugPlugin);
    app.add_plugins(UtilPlugin);

    app.insert_resource(ClearColor(Color::srgb(0.7, 0.7, 0.7)));
    app.run();

    Ok(())
}
