use avian2d::prelude::ExternalImpulse;
use bevy::{
    app::{App, Update},
    ecs::{
        component::Component,
        event::{EventReader, EventWriter},
        query::{Or, With},
        schedule::IntoSystemConfigs,
        system::Query,
    },
    reflect::Reflect,
    state::condition::in_state,
};
use bevy_stats::{
    statmod::{ModType, StatValueChange},
    ResourceChangeEvent, Stat,
};

use crate::{
    game::stats::{Damage, Health, Knockback},
    states::TimerState,
};

use super::{
    actors::Actor,
    events::{AttackEvent, DamageEvent, KnockbackEvent},
};

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Weapon;

pub fn weapon_plugin(app: &mut App) {
    app.add_event::<KnockbackEvent>()
        .add_event::<DamageEvent>()
        .add_event::<AttackEvent>();

    app.register_type::<KnockbackEvent>()
        .register_type::<DamageEvent>()
        .register_type::<AttackEvent>()
        .register_type::<Weapon>();

    app.add_systems(
        Update,
        (
            (knockback_from_attacks, impart_knockback).chain(),
            (damage_from_attacks, impart_damage).chain(),
        )
            .run_if(in_state(TimerState::Playing)),
    );
}

pub(crate) fn knockback_from_attacks(
    mut projectile_events: EventReader<AttackEvent>,
    mut knockback_events: EventWriter<KnockbackEvent>,
    weapons: Query<&Stat<Knockback>, Or<(With<Weapon>, With<Actor>)>>,
) {
    for AttackEvent {
        attacker,
        weapon,
        defender,
        location: _,
        direction,
    } in projectile_events.read()
    {
        if let Ok(knockback) = weapons.get(*weapon) {
            knockback_events.send(KnockbackEvent {
                entity: *defender,
                direction: *direction,
                force: knockback.current_value(),
            });
        }
    }
}

pub(crate) fn damage_from_attacks(
    mut damage_events: EventWriter<DamageEvent>,
    mut projectile_events: EventReader<AttackEvent>,
    damagers: Query<&Stat<Damage>, Or<(With<Weapon>, With<Actor>)>>,
) {
    for AttackEvent {
        attacker,
        weapon,
        defender,
        location: _,
        direction: _,
    } in projectile_events.read()
    {
        if let Ok(damage) = damagers.get(*weapon) {
            damage_events.send(DamageEvent {
                target: *defender,
                source: *attacker,
                amount: damage.current_value(),
            });
        }
    }
}

fn impart_knockback(
    mut knockback_events: EventReader<KnockbackEvent>,
    mut target_query: Query<&mut ExternalImpulse>,
) {
    for KnockbackEvent {
        entity,
        direction,
        force,
    } in knockback_events.read()
    {
        let impulse_vector = direction.normalize() * *force * 3000.;
        if let Ok(mut impulse) = target_query.get_mut(*entity) {
            impulse.apply_impulse(impulse_vector);
        }
    }
}

fn impart_damage(
    mut damage_events: EventReader<DamageEvent>,
    mut resource_changes: EventWriter<ResourceChangeEvent<Health>>,
) {
    for DamageEvent {
        target,
        source: _,
        amount,
    } in damage_events.read()
    {
        resource_changes.send(ResourceChangeEvent {
            change: StatValueChange::new(amount * -1., ModType::Offset),
            target: *target,
        });
    }
}
