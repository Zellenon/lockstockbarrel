use bevy::{
    prelude::{Added, Changed, Commands, Entity, Or, Query, Without},
    reflect::Reflect,
};
use bevy_stats;
use bevy_stats::{RPGResource, RPGStat, Resource, Stat};
use bevy_twin_stick::stats;

#[derive(Debug, Clone, Copy, Reflect)]
pub struct Health;
#[derive(Debug, Clone, Copy, Reflect)]
pub struct Speed;
#[derive(Debug, Clone, Copy, Reflect)]
pub struct Damage;
#[derive(Debug, Clone, Copy, Reflect)]
pub struct Knockback;

impl RPGStat for Health {}
impl RPGResource for Health {}
impl RPGStat for Speed {}
impl RPGStat for Damage {}
impl RPGStat for Knockback {}

pub(crate) fn ensure_health(
    mut commands: Commands,
    query: Query<(&Resource<Health>, Entity), Without<stats::Health>>,
) {
    for (stat, entity) in query.iter() {
        commands
            .get_entity(entity)
            .unwrap()
            .insert(stats::Health(stat.current_value()));
    }
}

pub(crate) fn ensure_speed(
    mut commands: Commands,
    query: Query<(&Stat<Speed>, Entity), Without<stats::Speed>>,
) {
    for (stat, entity) in query.iter() {
        commands
            .get_entity(entity)
            .unwrap()
            .insert(stats::Speed(stat.current_value()));
    }
}

pub(crate) fn sync_speed_to_speed(
    mut commands: Commands,
    mut query: Query<
        (Option<&mut stats::Speed>, &Stat<Speed>, Entity),
        Or<(Changed<Stat<Speed>>, Added<Stat<Speed>>)>,
    >,
) {
    for (twin_speed, speed, entity) in query.iter_mut() {
        if let Some(mut twin_speed) = twin_speed {
            twin_speed.0 = speed.current_value();
        } else {
            commands
                .get_entity(entity)
                .unwrap()
                .insert(Stat::<Speed>::new(speed.current_value()));
        }
    }
}

pub(crate) fn sync_health_to_health(
    mut query: Query<(&mut stats::Health, &Resource<Health>), Changed<Resource<Health>>>,
) {
    for (mut twin_health, health) in query.iter_mut() {
        twin_health.0 = health.current_value();
        println!("New health: {}", twin_health.0);
    }
}
