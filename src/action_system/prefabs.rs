use bevy_composable::tree::ComponentTree;

use super::{
    actions::{oneshot::oneshot, spawn::spawn},
    actuator::{actuator, ActuatorTrigger},
    triggers::{proximity::proximity, timer::timer},
};

pub fn spawn_delay(delay: f32, tree: ComponentTree) -> ComponentTree {
    actuator(ActuatorTrigger::RisingEdge, 0.5) + timer(delay) + spawn(tree) + oneshot()
}

pub fn spawn_prox(factions: u16, radius: f32, tree: ComponentTree) -> ComponentTree {
    actuator(ActuatorTrigger::RisingEdge, 0.5)
        + proximity(factions, radius)
        + spawn(tree)
        + oneshot()
}
