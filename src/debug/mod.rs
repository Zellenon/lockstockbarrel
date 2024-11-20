use bevy::{
    app::{Plugin, Update},
    prelude::KeyCode,
};
use bevy_editor_pls::EditorPlugin;
use bevy_egui::EguiSettings;
use grid::grid_system;

pub struct DebugPlugin;
pub mod grid;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(EguiSettings {
            scale_factor: 1.0,
            default_open_url_target: None,
        });
        // #[cfg(feature = "editor")]
        app.add_plugins(EditorPlugin::new())
            .insert_resource(default_editor_controls());

        #[cfg(feature = "physdebug")]
        app.add_plugin(RapierDebugRenderPlugin::default());

        app.add_systems(Update, grid_system);
    }
}

fn default_editor_controls() -> bevy_editor_pls::controls::EditorControls {
    use bevy_editor_pls::controls::*;
    let mut editor_controls = EditorControls::default_bindings();
    editor_controls.unbind(Action::PlayPauseEditor);
    editor_controls.insert(
        Action::PlayPauseEditor,
        Binding {
            input: UserInput::Single(Button::Keyboard(KeyCode::KeyQ)),
            conditions: vec![BindingCondition::ListeningForText(false)],
        },
    );
    editor_controls
}
