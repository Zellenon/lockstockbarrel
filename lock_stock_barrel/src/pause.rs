use bevy::{
    prelude::{Commands, Input, KeyCode, NextState, Query, Res, ResMut, With},
    window::PrimaryWindow,
};
use bevy_egui::{egui, EguiContext};

use crate::states::{AppState, InGameMenu};

pub(crate) fn pause_on_esc(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mut state: ResMut<NextState<InGameMenu>>,
) {
    if input.pressed(KeyCode::Escape) {
        state.set(InGameMenu::Pause)
    }
}

pub(crate) fn pause_gui(
    mut commands: Commands,
    mut root: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut menu_state: ResMut<NextState<InGameMenu>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    egui::Window::new("Pause")
        .resizable(false)
        .collapsible(false)
        .scroll2([false, false])
        .enabled(true)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::default())
        .show(root.single_mut().get_mut(), |ui| {
            if ui.button("Resume").clicked() {
                menu_state.set(InGameMenu::None);
            }
            if ui.button("Options").clicked() {
                menu_state.set(InGameMenu::Options);
            }
            if ui.button("Quit").clicked() {
                app_state.set(AppState::MainMenu);
                menu_state.set(InGameMenu::None);
            }
        });
}
