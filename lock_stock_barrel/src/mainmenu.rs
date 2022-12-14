use bevy::prelude::{Commands, ResMut};
use iyes_loopless::state::NextState;
use prettegui::bevy_egui::{egui, EguiContext};

use crate::states::AppState;

pub fn main_menu_gui(mut commands: Commands, mut root: ResMut<EguiContext>) {
    egui::CentralPanel::default().show(root.ctx_mut(), |ui| {
        ui.allocate_space(egui::Vec2::new(1.0, 100.0));

        egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label("LOCK STOCK & BARREL");
                if ui.button("Play Game").clicked() {
                    commands.insert_resource(NextState(AppState::Game));
                }
                if ui.button("Options").clicked() {
                    todo!();
                }
                if ui.button("Quit").clicked() {
                    commands.insert_resource(NextState(AppState::Exit));
                }
            })
        });

        if ui.button("Increment").clicked() {
            println!("pressed!");
        }

        // ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
        //     ui.add(egui::Hyperlink::from_label_and_url(
        //         "powered by egui",
        //         "https://github.com/emilk/egui/",
        //     ));
        // });
    });
}
