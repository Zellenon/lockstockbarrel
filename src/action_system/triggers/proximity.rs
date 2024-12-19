use bevy::{
    app::{App, Update},
    color::palettes::css::BLUE,
    math::Vec3Swizzles,
    prelude::{Commands, Component, Entity, Gizmos, Query, Transform, With, Without},
    reflect::Reflect,
};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree};
use core::f32;

use crate::{
    action_system::{
        actions::TelegraphedAction,
        actuator::{Actuator, ActuatorCondition},
    },
    twin_stick::actors::Faction,
};

#[derive(Component, Reflect, Clone, Debug)]
pub struct ProximityTrigger {
    pub triggering_factions: u16,
    pub radius: f32,
}

pub fn proximity(factions: u16, radius: f32) -> ComponentTree {
    ProximityTrigger::new(factions, radius).store()
}

impl ProximityTrigger {
    pub fn new(triggering_factions: u16, radius: f32) -> Self {
        Self {
            triggering_factions,
            radius,
        }
    }

    pub fn setup(app: &mut App) {
        app.register_type::<ProximityTrigger>();
        app.add_systems(
            Update,
            (
                activate_deactivate_proximity_triggers,
                display_prox_triggers,
            ),
        );
    }
}

pub fn activate_deactivate_proximity_triggers(
    prox_query: Query<
        (
            Entity,
            &Transform,
            &ProximityTrigger,
            Option<&ActuatorCondition>,
        ),
        With<Actuator>,
    >,
    triggering_entities: Query<(&Transform, &Faction), Without<ProximityTrigger>>,
    mut commands: Commands,
) {
    let (already_deactivated, already_activated): (Vec<_>, Vec<_>) = prox_query
        .iter()
        .map(|(entity, transform, trigger, option)| {
            (
                entity,
                triggering_entities
                    .iter()
                    .filter(|(_, fac)| ((1 << fac.0) & trigger.triggering_factions) != 0)
                    .map(|(t, _)| t.translation.xy().distance(transform.translation.xy()))
                    .fold(f32::INFINITY, |a, b| a.min(b)),
                trigger,
                option,
            )
        })
        .partition(|(_, _, _, option)| option.is_none());

    for (entity, min_distance, trigger, _) in already_deactivated.iter() {
        if min_distance < &trigger.radius {
            commands.entity(*entity).insert(ActuatorCondition);
        }
    }

    for (entity, min_distance, trigger, _) in already_activated.iter() {
        if min_distance > &trigger.radius {
            commands.entity(*entity).remove::<ActuatorCondition>();
        }
    }
}

pub fn display_prox_triggers(
    proximities: Query<(&ProximityTrigger, &Transform), With<TelegraphedAction>>,
    mut gizmos: Gizmos,
) {
    for (prox, transform) in proximities.iter() {
        gizmos.circle_2d(transform.translation.xy(), prox.radius, BLUE);
    }
}
