use actions::spawn::SpawnAction;
use actuator::Actuator;
use bevy::app::Plugin;
use triggers::timer::TimerTrigger;

pub mod actions;
pub mod actuator;
pub mod triggers;

pub struct ActionSystemPlugin;

impl Plugin for ActionSystemPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        Actuator::setup(app);

        TimerTrigger::setup(app);
        SpawnAction::setup(app);
    }
}
