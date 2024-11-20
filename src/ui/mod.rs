use bevy::prelude::IntoSystemConfigs;
use bevy::{
    app::{Plugin, Update},
    prelude::in_state,
};
use bevy_egui::EguiPlugin;
use hud::hud_gui;
use mainmenu::main_menu_gui;
use pausemenu::pause_gui;

use crate::states::{AppState, GameState, UIState};

pub mod hud;
pub mod mainmenu;
pub mod pausemenu;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(EguiPlugin);
        app.add_systems(Update, main_menu_gui.run_if(in_state(AppState::MainMenu)));

        app.add_systems(
            Update,
            pause_gui
                .run_if(in_state(UIState::Pause))
                .run_if(in_state(AppState::Game)),
        );
        app.add_systems(
            Update,
            hud_gui
                .run_if(in_state(GameState::InLevel))
                .run_if(in_state(AppState::Game)),
        );
    }
}
