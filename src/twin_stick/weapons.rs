use avian2d::prelude::{Forces, WriteRigidBodyForces};
use bevy::{
    app::{App, Update},
    ecs::schedule::IntoScheduleConfigs,
    ecs::{
        component::Component,
        query::{Or, With},
        system::Query,
    },
    prelude::{MessageReader, MessageWriter},
    reflect::Reflect,
    state::condition::in_state,
};
use bevy_stats::{
    statmod::{ModType, StatValueChange},
    ResourceChangeMessage, Stat,
};

use super::{
    actors::Actor,
    events::{AttackMessage, DamageMessage, KnockbackMessage},
};
use crate::{
    game::stats::{Damage, Health, Knockback},
    states::TimerState,
};

#[derive(Default, Component, Clone, Copy, Reflect, PartialEq, Eq, Hash, Debug)]
pub enum SpreadType {
    Spaced,
    NormalDistribution,
    Jittered,
    #[default]
    TrueRandom,
}

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Weapon;

pub fn weapon_plugin(app: &mut App) {
    app.add_message::<KnockbackMessage>()
        .add_message::<DamageMessage>()
        .add_message::<AttackMessage>();

    app.register_type::<KnockbackMessage>()
        .register_type::<DamageMessage>()
        .register_type::<AttackMessage>()
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
    mut projectile_messages: MessageReader<AttackMessage>,
    mut knockback_messages: MessageWriter<KnockbackMessage>,
    weapons: Query<&Stat<Knockback>, Or<(With<Weapon>, With<Actor>)>>,
) {
    for AttackMessage {
        attacker,
        weapon,
        defender,
        location: _,
        direction,
    } in projectile_messages.read()
    {
        if let Ok(knockback) = weapons.get(*weapon) {
            knockback_messages.write(KnockbackMessage {
                entity: *defender,
                direction: *direction,
                force: knockback.current_value(),
            });
        }
    }
}

pub(crate) fn damage_from_attacks(
    mut damage_messages: MessageWriter<DamageMessage>,
    mut projectile_messages: MessageReader<AttackMessage>,
    damagers: Query<&Stat<Damage>, Or<(With<Weapon>, With<Actor>)>>,
) {
    for AttackMessage {
        attacker,
        weapon,
        defender,
        location: _,
        direction: _,
    } in projectile_messages.read()
    {
        if let Ok(damage) = damagers.get(*weapon) {
            damage_messages.write(DamageMessage {
                target: *defender,
                source: *attacker,
                amount: damage.current_value(),
            });
        }
    }
}

fn impart_knockback(
    mut knockback_messages: MessageReader<KnockbackMessage>,
    mut target_query: Query<Forces>,
) {
    for KnockbackMessage {
        entity,
        direction,
        force,
    } in knockback_messages.read()
    {
        let impulse_vector = direction.normalize() * *force * 3000.;
        if let Ok(mut impulse) = target_query.get_mut(*entity) {
            impulse.apply_force(impulse_vector);
        }
    }
}

fn impart_damage(
    mut damage_messages: MessageReader<DamageMessage>,
    mut resource_changes: MessageWriter<ResourceChangeMessage<Health>>,
) {
    for DamageMessage {
        target,
        source: _,
        amount,
    } in damage_messages.read()
    {
        resource_changes.write(ResourceChangeMessage {
            change: StatValueChange::new(amount * -1., ModType::Offset),
            target: *target,
        });
    }
}
