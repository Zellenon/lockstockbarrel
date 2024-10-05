use bevy_egui::egui;
use bevy_egui::EguiContexts;

pub(crate) fn hud_gui(mut root: EguiContexts) {
    egui::Window::new("HUD1")
        .resizable(false)
        .collapsible(false)
        .scroll([false, false])
        .enabled(true)
        .anchor(egui::Align2::LEFT_TOP, egui::Vec2::new(50., 50.))
        .show(root.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Items:");
                let temp = vec!["Fireball", "Gun", "Turret", "Amulet", "", ""];
                for item in temp.into_iter() {
                    egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
                        ui.label(item);
                    });
                }
            })
        });
    egui::Window::new("HUD2")
        .resizable(false)
        .collapsible(false)
        .scroll([false, false])
        .enabled(true)
        .anchor(egui::Align2::LEFT_BOTTOM, egui::Vec2::default())
        .show(root.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Health:");
                ui.colored_label(egui::Color32::from_rgb(100, 255, 100), "000000XX");
            })
        });
    egui::Window::new("HUD3")
        .resizable(false)
        .collapsible(false)
        .scroll([false, false])
        .enabled(true)
        .anchor(egui::Align2::RIGHT_CENTER, egui::Vec2::default())
        .show(root.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                let temp = vec!["Faster", "AoE", "Dubler", "Eraser"];
                for item in temp.into_iter() {
                    egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
                        ui.label(item);
                    });
                }
            })
        });
}
