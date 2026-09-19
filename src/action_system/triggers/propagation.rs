use bevy::{
    app::{App, Update},
    ecs::{
        component::Component,
        entity::Entity,
        hierarchy::Children,
        query::{Added, With, Without},
        system::{Commands, Query},
    },
    prelude::RemovedComponents,
    reflect::Reflect,
};

use crate::action_system::actuator::ActuatorCondition;

#[derive(Component, Reflect, Clone, Debug)]
pub struct ChildOfTrigger;

impl ChildOfTrigger {
    pub fn setup(app: &mut App) {
        app.register_type::<ChildOfTrigger>();
        app.add_systems(Update, (trigger_with_parent, untrigger_with_parent));
    }
}

pub fn trigger_with_parent(
    parents: Query<&Children, Added<ActuatorCondition>>,
    triggers: Query<Entity, (With<ChildOfTrigger>, Without<ActuatorCondition>)>,
    mut commands: Commands,
) {
    for children in parents.iter() {
        let children = children.iter().filter_map(|e| triggers.get(*e).ok());
        for child in children {
            commands
                .get_entity(child)
                .unwrap()
                .insert(ActuatorCondition);
        }
    }
}

pub fn untrigger_with_parent(
    mut removed: RemovedComponents<ActuatorCondition>,
    parents: Query<&Children, Without<ActuatorCondition>>,
    triggers: Query<Entity, (With<ChildOfTrigger>, With<ActuatorCondition>)>,
    mut commands: Commands,
) {
    for children in removed.read().filter_map(|w| parents.get(w).ok()) {
        for child in children.iter().filter_map(|w| triggers.get(*w).ok()) {
            commands
                .get_entity(child)
                .unwrap()
                .remove::<ActuatorCondition>();
        }
    }
}
