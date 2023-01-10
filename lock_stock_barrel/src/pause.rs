use bevy::prelude::{Commands, Input, KeyCode, Res, ResMut};
use bevy_egui::{egui, EguiContext};
use iyes_loopless::state::NextState;

use crate::states::{AppState, InGameMenu};

pub(crate) fn pause_on_esc(mut commands: Commands, input: Res<Input<KeyCode>>) {
    if input.pressed(KeyCode::Escape) {
        commands.insert_resource(NextState(InGameMenu::Pause))
    }
}

pub(crate) fn pause_gui(mut commands: Commands, mut root: ResMut<EguiContext>) {
    egui::Window::new("Pause")
        .resizable(false)
        .collapsible(false)
        .scroll2([false, false])
        .enabled(true)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::default())
        .show(root.ctx_mut(), |ui| {
            if ui.button("Resume").clicked() {
                commands.insert_resource(NextState(InGameMenu::None));
            }
            if ui.button("Options").clicked() {
                commands.insert_resource(NextState(InGameMenu::Options));
            }
            if ui.button("Quit").clicked() {
                commands.insert_resource(NextState(AppState::MainMenu));
                commands.insert_resource(NextState(InGameMenu::None));
            }
        });
}
