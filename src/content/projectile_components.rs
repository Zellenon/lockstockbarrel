use bevy::{
    prelude::{Commands, Component, Entity, EventReader, EventWriter, Query, Res, With},
    time::Time,
};
use bevy_stats::{
    statmod::{ResourceChangeEvent, StatModifier, StatValueChange},
    DeleteStatMod, Resource, Stat,
};
use bevy_twin_stick::projectile::ProjectileImpactEvent;

use super::stats::{Damage, Health, Speed};

#[derive(Component, Clone, Copy)]
pub struct SlowOnImpact {
    pub strength: f32,
    pub decay: f32,
    pub threshhold: f32,
}

pub(crate) fn apply_slow_on_hit(
    mut commands: Commands,
    mut impacts: EventReader<ProjectileImpactEvent>,
    mut targets: Query<&mut Stat<Speed>>,
    projectiles: Query<&SlowOnImpact>,
) {
    for ProjectileImpactEvent {
        projectile,
        impacted,
    } in impacts.iter()
    {
        if let (Ok(projectile), Ok(mut impacted)) =
            (projectiles.get(*projectile), targets.get_mut(*impacted))
        {
            let id = commands
                .spawn((
                    StatValueChange::<Speed>::new(
                        projectile.strength,
                        bevy_stats::statmod::ModType::Multiplier,
                    ),
                    StatModifier,
                    *projectile,
                ))
                .id();
            (*impacted).add_mod(id);
        }
    }
}

pub(crate) fn tick_fading_slow(
    mut query: Query<(&mut StatValueChange<Speed>, &SlowOnImpact, Entity), With<StatModifier>>,
    time: Res<Time>,
    mut events: EventWriter<DeleteStatMod>,
) {
    for (mut change, slow, entity) in query.iter_mut() {
        if change.value.abs() < slow.threshhold.abs() {
            events.send(DeleteStatMod(entity));
        } else {
            change.value *= 1. - slow.decay * time.delta_seconds_f64() as f32;
        }
    }
}

pub(crate) fn damaging_projectile(
    mut targets: Query<(Entity, &mut Resource<Health>)>,
    attacks: Query<&Stat<Damage>>,
    mut events: EventReader<ProjectileImpactEvent>,
    mut damages: EventWriter<ResourceChangeEvent<Health>>,
) {
    for ProjectileImpactEvent {
        projectile,
        impacted,
    } in events.iter()
    {
        if let Ok(Stat::<Damage> {
            base: _, current, ..
        }) = attacks.get(*projectile)
        {
            if let Ok((_entity, _)) = targets.get_mut(*impacted) {
                println!("Damage");
                damages.send(ResourceChangeEvent {
                    change: StatValueChange::offset(-1. * current),
                    target: *impacted,
                })
            }
        }
    }
}
