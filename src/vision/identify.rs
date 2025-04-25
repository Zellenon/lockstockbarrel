use bevy::{
    ecs::system::{Query, Res},
    time::Time,
    utils::Entry,
};

use super::{Identifying, Tracking, LOS};

pub fn always_identify_tracked(mut trackers: Query<(&Tracking, &mut Identifying)>) {
    for (tracking, mut ident) in trackers.iter_mut() {
        for entity in tracking.0.iter() {
            match ident.0.entry(*entity) {
                Entry::Occupied(mut occupied_entry) => {
                    if *occupied_entry.get() < 100. {
                        occupied_entry.insert(100.);
                    }
                }
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(100.);
                }
            }
        }
    }
}

pub fn identify_los(mut seers: Query<(&LOS, &mut Identifying)>, time: Res<Time>) {
    for (los, mut identifying) in seers.iter_mut() {
        for target in los.0.iter() {
            match identifying.0.entry(*target) {
                Entry::Occupied(mut occupied_entry) => {
                    occupied_entry.insert(
                        (occupied_entry.get() + time.delta().as_secs_f32() * 10.).min(100.),
                    );
                }
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(0.);
                }
            }
        }
    }
}
