use bevy::{app::{App, Update}, ecs::{component::Component, entity::Entity, query::Changed, system::{Commands, Query}}, hierarchy::Children, reflect::Reflect, utils::HashMap};
use leafwing_input_manager::prelude::ActionState;
use strum::IntoEnumIterator;

use crate::{action_system::actuator::ActuatorCondition, twin_stick::ai::keyboard::PlayerAction};

#[derive(Component, Reflect, Clone, Debug)]
pub struct PlayerActionTrigger {
    activates_on: HashMap<PlayerAction,bool>
}

impl PlayerActionTrigger {
    pub fn new<const N: usize>(received_actions: [PlayerAction; N]) -> PlayerActionTrigger {
        PlayerActionTrigger { activates_on: PlayerAction::iter().map(|w| (w, received_actions.contains(&w))).collect() }
    }

    pub fn setup(app: &mut App) {
        app.register_type::<PlayerActionTrigger>();
        app.add_systems(Update, sync_playeraction_triggers);
    }
}

pub fn sync_playeraction_triggers(
    parents: Query<(&ActionState<PlayerAction>, &Children), Changed<ActionState<PlayerAction>>>,
    triggers: Query<(&PlayerActionTrigger, Option<&ActuatorCondition>)>,
    mut commands: Commands
) {
    for (actions, children) in parents.iter() {
        let children = children.iter()
            .map(|w| (w, triggers.get(*w)))
            .filter_map(|(e, trigger)|
                trigger.ok().map(|w| (e, w))
            );
        for (e, (filter, has_activated)) in children {
            if has_activated.is_some() {
                if !filter.activates_on.iter().any(|(action, used)| *used && actions.pressed(action)) {
                    commands.get_entity(*e).unwrap().remove::<ActuatorCondition>();
                }
            } else {
                if filter.activates_on.iter().any(|(action, used)| *used && actions.pressed(action)) {
                    commands.get_entity(*e).unwrap().insert(ActuatorCondition);
                }
            }

        }
    }
    //}
}
