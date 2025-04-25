use std::time::Duration;

use bevy::{
    ecs::system::{Query, Res},
    time::{Time, Timer, TimerMode},
    utils::Entry,
};

use super::{Spotting, LOS};

pub fn do_los_spotting(mut spotters: Query<(&mut Spotting, &LOS)>) {
    for (mut spotter, LOS(los)) in spotters.iter_mut() {
        for seen_obj in los.iter() {
            match spotter.0.entry(*seen_obj) {
                Entry::Occupied(occupied_entry) => occupied_entry.into_mut().reset(),
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(Timer::new(Duration::from_secs(3), TimerMode::Once));
                }
            };
        }
    }
}

pub fn remove_expired_spots(mut query: Query<&mut Spotting>) {
    for mut spots in query.iter_mut() {
        spots.0 = spots
            .0
            .iter()
            .filter(|(e, timer)| !timer.finished())
            .map(|(e, timer)| (*e, timer.clone()))
            .collect()
    }
}

pub fn tick_spotting(mut query: Query<&mut Spotting>, time: Res<Time>) {
    for mut spots in query.iter_mut() {
        for (_, timer) in spots.0.iter_mut() {
            timer.tick(time.delta());
        }
    }
}
