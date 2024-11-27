use bevy::{
    app::App,
    prelude::{Commands, Component, Trigger},
    reflect::Reflect,
};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree};

use crate::{action_system::actuator::Actuate, util::add_observer_to_component};

#[derive(Component, Clone, Reflect, Debug, Copy)]
pub struct OneShotAction;

impl OneShotAction {
    pub fn setup(app: &mut App) {
        app.register_type::<OneShotAction>();
        app.observe(add_observer_to_component::<OneShotAction, _, _, _, _>(
            despawn_oneshot,
        ));
    }
}

pub fn oneshot() -> ComponentTree {
    OneShotAction.store()
}

pub fn despawn_oneshot(trigger: Trigger<Actuate>, mut commands: Commands) {
    commands.get_entity(trigger.entity()).unwrap().despawn();
}
