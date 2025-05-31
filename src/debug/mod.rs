use arrows::{display_arrows, Arrows};
use bevy::{
    app::{Plugin, Update},
    color::palettes::css::{GREEN, RED},
    ecs::{query::With, system::Query},
    gizmos::gizmos::Gizmos,
    input::keyboard::KeyCode,
    math::{Isometry2d, Vec2, Vec3Swizzles},
    transform::components::GlobalTransform,
};
use bevy_editor_pls::prelude::EditorPlugin;
use grid::grid_system;

use crate::action_system::{
    actuator::Actuator,
    triggers::{key_action::PlayerActionTrigger, propagation::ParentTrigger},
};

pub struct DebugPlugin;
pub mod arrows;
pub mod grid;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // #[cfg(feature = "editor")]
        app.add_plugins(EditorPlugin::new());
        app.init_resource::<Arrows>();
        //    .insert_resource(default_editor_controls());

        #[cfg(feature = "physdebug")]
        app.add_plugin(RapierDebugRenderPlugin::default());

        app.add_systems(Update, (test_display, grid_system, display_arrows));
    }
}

fn test_display(weapons: Query<&GlobalTransform, (With<ParentTrigger>)>, mut gizmos: Gizmos) {
    for transform in weapons.iter() {
        gizmos.circle_2d(transform.compute_transform().translation.xy(), 10., RED);
    }
}

//fn default_editor_controls() -> bevy_editor_pls::controls::EditorControls {
//    use bevy_editor_pls::controls::*;
//    let mut editor_controls = EditorControls::default_bindings();
//    editor_controls.unbind(Action::PlayPauseEditor);
//    editor_controls.insert(
//        Action::PlayPauseEditor,
//        Binding {
//            input: UserInput::Single(Button::Keyboard(KeyCode::KeyQ)),
//            conditions: vec![BindingCondition::ListeningForText(false)],
//        },
//    );
//    editor_controls
//}
