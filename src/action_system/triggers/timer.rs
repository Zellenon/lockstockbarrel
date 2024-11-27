use bevy_composable::app_impl::ComponentTreeable;
use std::time::Duration;

use bevy::{
    app::{App, Update},
    color::palettes::css::RED,
    math::Vec3Swizzles,
    prelude::{Changed, Commands, Component, Entity, Gizmos, Query, Res, Transform, With, Without},
    reflect::Reflect,
    time::{Time, Timer},
};
use bevy_composable::tree::ComponentTree;

use crate::action_system::actuator::{Actuator, ActuatorCondition};

#[derive(Component, Reflect, Clone, Debug)]
pub struct TimerTrigger {
    pub timer: Timer,
}

impl TimerTrigger {
    pub fn new(duration: f32) -> Self {
        TimerTrigger {
            timer: Timer::new(
                Duration::from_secs_f32(duration),
                bevy::time::TimerMode::Once,
            ),
        }
    }

    pub fn setup(app: &mut App) {
        app.register_type::<TimerTrigger>();
        app.add_systems(
            Update,
            (
                tick_timer_triggers,
                activate_timer_triggers,
                deactivate_timer_triggers,
                display_timer_triggers,
            ),
        );
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

pub fn display_timer_triggers(timers: Query<(&TimerTrigger, &Transform)>, mut gizmos: Gizmos) {
    for (timer, transform) in timers.iter() {
        gizmos.circle_2d(
            transform.translation.xy(),
            (1. - timer.timer.fraction()) * 50.,
            RED,
        );
    }
}
