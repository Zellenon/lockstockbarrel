use actions::{oneshot::OneShotAction, spawn::SpawnAction, vel_spawn::VelSpawnAction};
use actuator::Actuator;
use bevy::{app::{Plugin, Update}, ecs::schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet}, reflect::Reflect};
use triggers::{key_action::PlayerActionTrigger, propagation::ParentTrigger, proximity::ProximityTrigger, timer::TimerTrigger};

pub mod actions;
pub mod actuator;
pub mod prefabs;
pub mod triggers;

pub struct ActionSystemPlugin;

#[derive(Reflect, SystemSet, Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Ord, Eq)]
pub enum ActuatorLogicPhases {
    PreActuate,
    Actuate,
    PostActuate
}

impl Plugin for ActionSystemPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        Actuator::setup(app);

        TimerTrigger::setup(app);
        ProximityTrigger::setup(app);
        PlayerActionTrigger::setup(app);
        ParentTrigger::setup(app);

        SpawnAction::setup(app);
        OneShotAction::setup(app);
        VelSpawnAction::setup(app);

        app.configure_sets(Update, (
            ActuatorLogicPhases::PreActuate,
            ActuatorLogicPhases::Actuate.after(ActuatorLogicPhases::PreActuate),
            ActuatorLogicPhases::PostActuate.after(ActuatorLogicPhases::Actuate)
        ));
    }
}
