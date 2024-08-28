use bevy::app::Plugin;
use bevy_egui::EguiPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        #[cfg(not(feature = "editor"))]
        app.add_plugins(EguiPlugin);
    }
}
