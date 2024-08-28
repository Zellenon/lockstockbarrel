use bevy::app::Plugin;
#[cfg(feature = "inspect")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        #[cfg(feature = "inspect")]
        app.add_plugins(WorldInspectorPlugin::new());

        #[cfg(feature = "physdebug")]
        app.add_plugin(RapierDebugRenderPlugin::default());
    }
}
