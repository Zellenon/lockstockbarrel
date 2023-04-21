use bevy::{
    prelude::{Commands, Component, Entity, EventReader, EventWriter, Query, Res, With},
    time::Time,
};
use bevy_stats::{
    statmod::{StatModifier, StatValueChange},
    Stat,
};
use twin_stick::projectile::ProjectileImpactEvent;

use super::stats::{DeleteStatMod, Speed};

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
                        bevy_stats::statmod::ModType::Multiplier,
                        projectile.strength,
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
