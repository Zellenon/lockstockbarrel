use bevy::{app::{App, Update}, ecs::{component::Component, entity::Entity, query::{Added, With, Without}, removal_detection::RemovedComponents, system::{Commands, Query}}, hierarchy::Children, reflect::Reflect};

use crate::action_system::actuator::ActuatorCondition;

#[derive(Component, Reflect, Clone, Debug)]
pub struct ParentTrigger;

impl ParentTrigger {
    pub fn setup(app: &mut App) {
        app.register_type::<ParentTrigger>();
        app.add_systems(Update, (trigger_with_parent, untrigger_with_parent));
    }
}

pub fn trigger_with_parent(
    parents: Query<&Children, Added<ActuatorCondition>>,
    triggers: Query<Entity, (With<ParentTrigger>,Without<ActuatorCondition>)>,
    mut commands: Commands
) {
    for children in parents.iter() {
        let children = children.iter().filter_map(|e| triggers.get(*e).ok());
        for child in children {
            commands.get_entity(child).unwrap().insert(ActuatorCondition);
        }
    }
}

pub fn untrigger_with_parent(
    mut removed: RemovedComponents<ActuatorCondition>,
    parents: Query<&Children, Without<ActuatorCondition>>,
    triggers: Query<Entity, (With<ParentTrigger>,With<ActuatorCondition>)>,
    mut commands: Commands
) {
    for children in removed.read().filter_map(|w| parents.get(w).ok()) {
        for child in children.iter().filter_map(|w| triggers.get(*w).ok()) {
            commands.get_entity(child).unwrap().remove::<ActuatorCondition>();
        }
    }
}
