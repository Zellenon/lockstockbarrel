use bevy::{
    app::{App, Update},
    ecs::schedule::IntoSystemConfigs,
    prelude::{Added, Changed, Commands, Component, Entity, Event, Query, Res, Trigger},
    reflect::Reflect,
    time::{Time, Timer},
};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree};
use std::time::Duration;

use super::ActuatorLogicPhases;
use crate::util::add_observer_to_component;

#[derive(Event, Reflect, Debug)]
pub struct Actuate;
#[derive(Event, Reflect, Debug)]
pub struct ActuatorCooldownFinished;

#[derive(Reflect, Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum ActuatorFireStyle {
    Constantly,
    RisingEdge,
    StoreConstantly(bool),
    StoreRisingEdge(bool),
    SemiAuto(bool),
}

#[derive(Component, Reflect, Clone, Debug)]
pub struct Actuator {
    pub fire_style: ActuatorFireStyle,
    pub cooldown: Timer,
}

impl Actuator {
    pub fn new(fire_style: ActuatorFireStyle, cooldown_secs: f32) -> Self {
        Self {
            fire_style,
            cooldown: Timer::new(
                Duration::from_secs_f32(cooldown_secs),
                bevy::time::TimerMode::Once,
            ),
        }
    }

    pub fn setup(app: &mut App) {
        app.register_type::<Actuator>();
        app.add_systems(
            Update,
            (tick_actuator_cooldown, fire_actuator_on_condition_change)
                .in_set(ActuatorLogicPhases::PreActuate),
        );
        app.add_systems(
            Update,
            (fire_actuator_on_cooldown_over,).in_set(ActuatorLogicPhases::Actuate),
        );
        app.add_observer(add_observer_to_component::<Actuator, _, _, _, _>(
            actuator_cooldown_on_actuate,
        ));
    }
}

pub fn actuator(trigger: ActuatorFireStyle, cooldown: f32) -> ComponentTree {
    Actuator {
        fire_style: trigger,
        cooldown: {
            let mut a = Timer::new(
                Duration::from_secs_f32(cooldown),
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
        match act.fire_style {
            ActuatorFireStyle::Constantly => {
                if act.cooldown.finished() {
                    commands.trigger_targets(Actuate, e);
                }
            }
            ActuatorFireStyle::RisingEdge => {
                if act.cooldown.finished() {
                    commands.trigger_targets(Actuate, e);
                }
            }
            ActuatorFireStyle::StoreConstantly(_) => {
                if act.cooldown.finished() {
                    commands.trigger_targets(Actuate, e);
                } else {
                    act.fire_style = ActuatorFireStyle::StoreConstantly(true);
                }
            }
            ActuatorFireStyle::StoreRisingEdge(_) => {
                if act.cooldown.finished() {
                    commands.trigger_targets(Actuate, e);
                    act.fire_style = ActuatorFireStyle::StoreRisingEdge(false);
                } else {
                    act.fire_style = ActuatorFireStyle::StoreRisingEdge(true);
                }
            }
            ActuatorFireStyle::SemiAuto(_) => {
                if act.cooldown.finished() {
                    commands.trigger_targets(Actuate, e);
                    act.fire_style = ActuatorFireStyle::SemiAuto(false);
                } else {
                    act.fire_style = ActuatorFireStyle::SemiAuto(true);
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
        match act.fire_style {
            ActuatorFireStyle::Constantly => {
                if condition.is_some() {
                    commands.trigger_targets(Actuate, e);
                }
            }
            ActuatorFireStyle::RisingEdge => (),
            ActuatorFireStyle::StoreConstantly(cond) => {
                if cond {
                    commands.trigger_targets(Actuate, e);
                    if condition.is_none() {
                        act.fire_style = ActuatorFireStyle::StoreConstantly(false);
                    }
                }
            }
            ActuatorFireStyle::StoreRisingEdge(cond) => {
                if cond {
                    commands.trigger_targets(Actuate, e);
                    act.fire_style = ActuatorFireStyle::StoreRisingEdge(false);
                }
            }
            ActuatorFireStyle::SemiAuto(cond) => {
                if cond {
                    if condition.is_none() {
                        commands.trigger_targets(Actuate, e);
                        act.fire_style = ActuatorFireStyle::SemiAuto(false);
                    }
                } else {
                    act.fire_style = ActuatorFireStyle::SemiAuto(false)
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
