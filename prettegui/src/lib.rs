use bevy::{prelude::*, window::PrimaryWindow};
pub extern crate bevy_egui;
use bevy_egui::EguiContexts;
pub use bevy_egui::{
    egui::{self, TextureId},
    EguiContext, EguiPlugin,
};
use bevy_ninepatch::NinePatchPlugin;
use nice_image_button::TextImageButton;

pub struct GUIPlugin;

pub mod nice_image_button;

impl Plugin for GUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(NinePatchPlugin::<()>::default());
        app.add_plugin(EguiPlugin);
        app.add_system(ui_test);
    }
}

fn ui_test(mut egui: EguiContexts) {
    egui::Window::new("Hello").show(egui.ctx_mut(), |ui| {
        ui.label("world");
        ui.add(TextImageButton::new(
            TextureId::default(),
            TextureId::default(),
            egui::Vec2::new(50., 20.),
            Some("Text shit".into()),
        ))
    });
}
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
