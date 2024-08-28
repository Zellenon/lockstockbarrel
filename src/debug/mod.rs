use bevy::app::Plugin;
#[cfg(feature = "editor")]
use bevy_editor_pls::EditorPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        #[cfg(feature = "editor")]
        app.add_plugins(EditorPlugin::default());

        #[cfg(feature = "physdebug")]
        app.add_plugin(RapierDebugRenderPlugin::default());
    }
}
