use bevy::ecs::bundle::Bundle;

use super::{
    actions::{oneshot::OneShotAction, spawn::spawn},
    actuator::{actuator, ActuatorFireStyle},
    triggers::{proximity::proximity, timer::timer},
};

pub fn spawn_delay(delay: f32, bundle: impl Bundle) -> impl Bundle {
    actuator(ActuatorFireStyle::RisingEdge, 0.5) + timer(delay) + spawn(bundle) + OneShotAction
}

pub fn spawn_prox(factions: u16, radius: f32, bundle: impl Bundle) -> impl Bundle {
    actuator(ActuatorFireStyle::RisingEdge, 0.5)
        + proximity(factions, radius)
        + spawn(bundle)
        + OneShotAction
}
