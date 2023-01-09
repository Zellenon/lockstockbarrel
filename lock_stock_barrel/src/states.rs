use bevy::{
    app::AppExit,
    prelude::{Commands, EventWriter, Input, KeyCode, Plugin, Res, SystemSet},
};
use iyes_loopless::prelude::*;

use crate::mainmenu::main_menu_gui;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_loopless_state(AppState::MainMenu);
        app.add_loopless_state(GameState::PlayingArena);
        app.add_system(exit.run_in_state(AppState::Exit));
        // app.add_system(test_system);
        app.add_system(main_menu_gui.run_in_state(AppState::MainMenu));

        // app.add_system(main_menu_gui);
    }
}

fn test_system(mut commands: Commands, input: Res<Input<KeyCode>>) {
    if input.pressed(KeyCode::Space) {
        commands.insert_resource(NextState(AppState::Exit));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    MainMenu,
    Game,
    Pause,
    Exit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    PlayingArena,
    InMenu(InGameMenu),
    Map,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InGameMenu {
    Inventory,
    Pause,
}

fn exit(mut app_exit_events: EventWriter<AppExit>) {
    app_exit_events.send(AppExit);
}
