use bevy::{
    app::App,
    ecs::{event::Trigger, observer::On},
    prelude::{Commands, Component},
    reflect::Reflect,
};

use crate::{action_system::actuator::Actuate, util::add_observer_to_component};

#[derive(Component, Clone, Reflect, Debug, Copy)]
pub struct OneShotAction;

impl OneShotAction {
    pub fn setup(app: &mut App) {
        app.register_type::<OneShotAction>();
        app.add_observer(add_observer_to_component::<OneShotAction, _, _, _, _>(
            despawn_oneshot,
        ));
    }
}

pub fn despawn_oneshot(trigger: On<Actuate>, mut commands: Commands) {
    commands.get_entity(trigger.event().0).unwrap().despawn();
}
