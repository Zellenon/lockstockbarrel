use bevy::{
    app::App,
    prelude::{Commands, Component, Query, Transform, Trigger},
};
use bevy_composable::{
    app_impl::{ComplexSpawnable, ComponentTreeable},
    tree::ComponentTree,
};

use crate::{action_system::actuator::Actuate, util::add_observer_to_component};

#[derive(Component, Clone)]
pub struct SpawnAction {
    pub payload: Vec<ComponentTree>,
}

impl SpawnAction {
    pub fn spawn(tree: ComponentTree) -> Self {
        Self {
            payload: vec![tree],
        }
    }

    pub fn spawns<T: Iterator<Item = ComponentTree>>(trees: T) -> Self {
        Self {
            payload: trees.collect(),
        }
    }

    pub fn setup(app: &mut App) {
        // app.register_type::<SpawnAction>();
        app.observe(add_observer_to_component::<SpawnAction, _, _, _, _>(
            do_spawn_action,
        ));
    }
}

pub fn spawn(tree: ComponentTree) -> ComponentTree {
    SpawnAction::spawn(tree).store()
}

pub fn spawns<T: Iterator<Item = ComponentTree>>(trees: T) -> ComponentTree {
    SpawnAction::spawns(trees).store()
}

pub fn do_spawn_action(
    trigger: Trigger<Actuate>,
    spawners: Query<(&SpawnAction, &Transform)>,
    mut commands: Commands,
) {
    if let Ok((spawn_action, transform)) = spawners.get(trigger.entity()) {
        for payload in spawn_action.payload.iter() {
            commands.compose(
                payload.clone() + Transform::from_translation(transform.translation).store(),
            );
        }
    }
}
