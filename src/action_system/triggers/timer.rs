use bevy::{
    app::{App, Update},
    color::palettes::css::RED,
    math::Vec3Swizzles,
    prelude::{
        Changed, Commands, Component, Entity, Gizmos, Query, Res, Transform, Trigger, With, Without,
    },
    reflect::Reflect,
    time::{Time, Timer},
};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree};
use std::time::Duration;

use crate::{
    action_system::{
        actions::TelegraphedAction,
        actuator::{Actuator, ActuatorCondition, ActuatorCooldownFinished},
    },
    util::add_observer_to_component,
};

#[derive(Reflect, Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum TimerResetMode {
    Immediate,
    ActuatorCooldown,
}

#[derive(Component, Reflect, Clone, Debug)]
pub struct TimerTrigger {
    pub timer: Timer,
    pub reset_mode: TimerResetMode,
}

impl TimerTrigger {
    pub fn new(duration: f32) -> Self {
        TimerTrigger {
            timer: Timer::new(
                Duration::from_secs_f32(duration),
                bevy::time::TimerMode::Once,
            ),
            reset_mode: TimerResetMode::ActuatorCooldown,
        }
    }

    pub fn setup(app: &mut App) {
        app.register_type::<TimerTrigger>();
        app.add_systems(
            Update,
            (
                tick_timer_triggers,
                reset_immediate_timers,
                activate_timer_triggers,
                deactivate_timer_triggers,
                display_timer_triggers,
            ),
        );
        app.add_observer(add_observer_to_component::<TimerTrigger, _, _, _, _>(
            reset_actuator_timers,
        ));
    }
}

pub fn timer(duration: f32) -> ComponentTree {
    TimerTrigger::new(duration).store()
}

pub fn tick_timer_triggers(mut query: Query<&mut TimerTrigger>, time: Res<Time>) {
    let delta = time.delta();
    for mut timer in query.iter_mut() {
        if !timer.timer.finished() {
            timer.timer.tick(delta);
        }
    }
}

pub fn activate_timer_triggers(
    timers: Query<
        (Entity, &TimerTrigger),
        (
            Without<ActuatorCondition>,
            With<Actuator>,
            Changed<TimerTrigger>,
        ),
    >,
    mut commands: Commands,
) {
    for (entity, _) in timers
        .iter()
        .filter(|(_, trigger)| trigger.timer.just_finished())
    {
        commands.entity(entity).insert(ActuatorCondition);
    }
}

pub fn reset_immediate_timers(mut timers: Query<&mut TimerTrigger>) {
    for mut timer in timers
        .iter_mut()
        .filter(|timer| timer.reset_mode == TimerResetMode::Immediate)
        .filter(|trigger| trigger.timer.just_finished())
    {
        timer.timer.reset();
    }
}

pub fn reset_actuator_timers(
    trigger: Trigger<ActuatorCooldownFinished>,
    mut timers: Query<&mut TimerTrigger>,
) {
    timers.get_mut(trigger.entity()).unwrap().timer.reset()
}

pub fn deactivate_timer_triggers(
    timers: Query<
        (Entity, &TimerTrigger),
        (
            With<ActuatorCondition>,
            With<Actuator>,
            Changed<TimerTrigger>,
        ),
    >,
    mut commands: Commands,
) {
    for (entity, _) in timers
        .iter()
        .filter(|(_, trigger)| !trigger.timer.finished())
    {
        commands.entity(entity).remove::<ActuatorCondition>();
    }
}

pub fn display_timer_triggers(
    timers: Query<(&TimerTrigger, &Transform), With<TelegraphedAction>>,
    mut gizmos: Gizmos,
) {
    for (timer, transform) in timers.iter() {
        gizmos.circle_2d(
            transform.translation.xy(),
            (1. - timer.timer.fraction()) * 50.,
            RED,
        );
    }
}
