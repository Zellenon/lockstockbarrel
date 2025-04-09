use bevy_composable::app_impl::ComponentTreeable;
use std::{time::Duration, u64};

use bevy::{
    app::{App, Update},
    prelude::{Added, Changed, Commands, Component, Entity, Event, Query, Res, Trigger},
    reflect::Reflect,
    time::{Time, Timer},
};
use bevy_composable::tree::ComponentTree;

use crate::util::add_observer_to_component;

#[derive(Event, Reflect, Debug)]
pub struct Actuate;
#[derive(Event, Reflect, Debug)]
pub struct ActuatorCooldownFinished;

#[derive(Reflect, Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum ActuatorTrigger {
    Constantly,
    RisingEdge,
    StoreConstantly(bool),
    StoreRisingEdge(bool),
}

#[derive(Component, Reflect, Clone, Debug)]
pub struct Actuator {
    pub trigger_type: ActuatorTrigger,
    pub cooldown: Timer,
}

impl Actuator {
    pub fn setup(app: &mut App) {
        app.register_type::<Actuator>();
        app.add_systems(
            Update,
            (
                tick_actuator_cooldown,
                fire_actuator_on_condition_change,
                fire_actuator_on_cooldown_over,
            ),
        );
        app.add_observer(add_observer_to_component::<Actuator, _, _, _, _>(
            actuator_cooldown_on_actuate,
        ));
    }
}

pub fn actuator(trigger: ActuatorTrigger, duration: f32) -> ComponentTree {
    Actuator {
        trigger_type: trigger,
        cooldown: {
            let mut a = Timer::new(
                Duration::from_secs_f32(duration),
                bevy::time::TimerMode::Once,
            );
            a.set_elapsed(Duration::from_secs(u64::MAX));
            a
        },
    }
    .store()
}

#[derive(Component, Reflect, Clone, Debug)]
pub struct ActuatorCondition;

pub fn tick_actuator_cooldown(mut actuators: Query<&mut Actuator>, time: Res<Time>) {
    let delta = time.delta();
    for mut act in actuators.iter_mut() {
        if !act.cooldown.finished() {
            act.cooldown.tick(delta);
        }
    }
}

pub fn fire_actuator_on_condition_change(
    mut actuators: Query<(Entity, &mut Actuator), Added<ActuatorCondition>>,
    mut commands: Commands,
) {
    for (e, mut act) in actuators.iter_mut() {
        match act.trigger_type {
            ActuatorTrigger::Constantly => {
                if act.cooldown.finished() {
                    commands.trigger_targets(Actuate, e);
                }
            }
            ActuatorTrigger::RisingEdge => {
                if act.cooldown.finished() {
                    commands.trigger_targets(Actuate, e);
                }
            }
            ActuatorTrigger::StoreConstantly(_) => {
                if act.cooldown.finished() {
                    commands.trigger_targets(Actuate, e);
                } else {
                    act.trigger_type = ActuatorTrigger::StoreConstantly(true);
                }
            }
            ActuatorTrigger::StoreRisingEdge(_) => {
                if act.cooldown.finished() {
                    commands.trigger_targets(Actuate, e);
                    act.trigger_type = ActuatorTrigger::StoreRisingEdge(false);
                } else {
                    act.trigger_type = ActuatorTrigger::StoreRisingEdge(true);
                }
            }
        }
    }
}

pub fn fire_actuator_on_cooldown_over(
    mut actuators: Query<(Entity, &mut Actuator, Option<&ActuatorCondition>), Changed<Actuator>>,
    mut commands: Commands,
) {
    for (e, mut act, condition) in actuators
        .iter_mut()
        .filter(|(_, act, _)| act.cooldown.just_finished())
    {
        commands.trigger_targets(ActuatorCooldownFinished, e);
        match act.trigger_type {
            ActuatorTrigger::Constantly => {
                if condition.is_some() {
                    commands.trigger_targets(Actuate, e);
                }
            }
            ActuatorTrigger::RisingEdge => (),
            ActuatorTrigger::StoreConstantly(cond) => {
                if cond {
                    commands.trigger_targets(Actuate, e);
                    if condition.is_none() {
                        act.trigger_type = ActuatorTrigger::StoreConstantly(false);
                    }
                }
            }
            ActuatorTrigger::StoreRisingEdge(cond) => {
                if cond {
                    commands.trigger_targets(Actuate, e);
                    act.trigger_type = ActuatorTrigger::StoreRisingEdge(false);
                }
            }
        }
    }
}

pub fn actuator_cooldown_on_actuate(
    trigger: Trigger<Actuate>,
    mut actuators: Query<&mut Actuator>,
) {
    actuators
        .get_mut(trigger.entity())
        .unwrap()
        .cooldown
        .reset();
}
