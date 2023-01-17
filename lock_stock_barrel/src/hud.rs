use bevy::prelude::{Commands, ResMut};
use prettegui::bevy_egui::{egui, EguiContext};

pub(crate) fn hud_gui(mut commands: Commands, mut root: ResMut<EguiContext>) {
    egui::Window::new("HUD1")
        .resizable(false)
        .collapsible(false)
        .scroll2([false, false])
        .enabled(true)
        .anchor(egui::Align2::LEFT_TOP, egui::Vec2::default())
        .show(root.ctx_mut(), |ui| {
            ui.label("Items:");
        });
}
