use std::marker::PhantomData;

use bevy::prelude::*;
pub use stat::{RPGResource, RPGStat, Stat};
mod app_impl;
mod stat;
mod statmod;

pub struct StatPlugin;

impl Plugin for StatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StatChangeEvent<Health>>()
            .add_system(do_stat_change::<Health>);
    }
}

pub struct StatChangeEvent<T>
where
    T: RPGStat,
{
    pub target: Entity,
    pub amount: f32,
    pub phantom: PhantomData<T>,
}

fn do_stat_change<T>(
    mut change_events: EventReader<StatChangeEvent<T>>,
    mut targets: Query<&mut Stat<T>>,
) where
    T: RPGStat,
{
    for StatChangeEvent {
        target,
        amount,
        phantom: _,
    } in change_events.iter()
    {
        let mut target_stat = targets.get_mut(*target).unwrap();
        target_stat.current = target_stat.current_value() + amount;
    }
}

pub struct Speed;
impl RPGStat for Speed {}

pub struct Health;
impl RPGStat for Health {}
impl RPGResource for Health {
    fn can_overmax() -> bool {
        true
    }
}
