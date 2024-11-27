use actions::{oneshot::OneShotAction, spawn::SpawnAction};
use actuator::Actuator;
use bevy::app::Plugin;
use triggers::{proximity::ProximityTrigger, timer::TimerTrigger};

pub mod actions;
pub mod actuator;
pub mod prefabs;
pub mod triggers;

pub struct ActionSystemPlugin;

impl Plugin for ActionSystemPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        Actuator::setup(app);

        TimerTrigger::setup(app);
        ProximityTrigger::setup(app);

        SpawnAction::setup(app);
        OneShotAction::setup(app);
    }
}
