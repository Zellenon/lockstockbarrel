use std::time::Duration;

use bevy::{
    app::{App, FixedUpdate},
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::{Changed, With},
        schedule::IntoSystemConfigs,
        system::{Query, Res},
    },
    reflect::Reflect,
    time::{Time, Timer, TimerMode},
    utils::{Entry, HashMap},
};
use bevy_stats::Stat;

use crate::{game::stats::SpotTime, twin_stick::events::AttackEvent};

use super::{VisionObjects, VisionSystems, LOS};

#[derive(Component, Default, Reflect, Clone, Debug)]
pub struct Spotting(pub HashMap<Entity, Timer>);

#[derive(Event, Clone, Copy, PartialEq, Reflect, Debug)]
pub struct StartSpottingEvent {
    pub spotter: Entity,
    pub target: Entity,
    pub spot_time: f32,
}

pub fn spotting_plugin(app: &mut App) {
    app.register_type::<Spotting>()
        .register_type::<StartSpottingEvent>();
    app.add_event::<StartSpottingEvent>();

    app.add_systems(
        FixedUpdate,
        (
            (
                do_los_spotting,
                (tick_spotting, remove_expired_spots).chain(),
                do_spot_attacks,
            ),
            process_spot_events,
        )
            .chain()
            .in_set(VisionSystems::SpotTrack),
    );
}

//TODO: There has to be a more efficient way to do this
pub fn do_los_spotting(
    spotters: Query<(Entity, &Stat<SpotTime>, &LOS), Changed<LOS>>,
    mut events: EventWriter<StartSpottingEvent>,
) {
    for (e, stat, LOS(los)) in spotters.iter() {
        for seen_obj in los.iter() {
            events.send(StartSpottingEvent {
                spotter: e,
                target: *seen_obj,
                spot_time: stat.current_value(),
            });
        }
    }
}

pub fn remove_expired_spots(mut query: Query<&mut Spotting>) {
    for mut spots in query.iter_mut() {
        spots.0 = spots
            .0
            .iter()
            .filter(|(_e, timer)| !timer.finished())
            .map(|(e, timer)| (*e, timer.clone()))
            .collect()
    }
}

pub fn tick_spotting(mut query: Query<(&mut Spotting, &LOS)>, time: Res<Time>) {
    for (mut spots, los) in query.iter_mut() {
        for (_, timer) in spots.0.iter_mut().filter(|w| !los.0.contains(w.0)) {
            timer.tick(time.delta());
        }
    }
}

pub fn process_spot_events(
    mut events: EventReader<StartSpottingEvent>,
    mut spotters: Query<&mut Spotting>,
) {
    for event in events.read() {
        if let Ok(mut spotter) = spotters.get_mut(event.spotter) {
            match spotter.0.entry(event.target) {
                Entry::Occupied(mut occupied_entry) => {
                    if occupied_entry.get().remaining_secs() < event.spot_time {
                        occupied_entry
                            .insert(Timer::from_seconds(event.spot_time, TimerMode::Once));
                    }
                }
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(Timer::from_seconds(event.spot_time, TimerMode::Once));
                }
            }
        }
    }
}

pub fn do_spot_attacks(
    mut attack_events: EventReader<AttackEvent>,
    mut spot_events: EventWriter<StartSpottingEvent>,
    spotters: Query<Entity, With<Spotting>>,
    spot_attacks: Query<(Entity, &Stat<SpotTime>)>,
    vision_objects: Query<Entity, VisionObjects>,
) {
    for AttackEvent {
        attacker,
        weapon,
        defender,
        location,
        direction,
    } in attack_events.read()
    {
        if let Ok((attack, attack_stat)) = spot_attacks.get(*weapon) {
            if let Ok(_) = vision_objects.get(*defender) {
                if let Ok(_) = spotters.get(*attacker) {
                    spot_events.send(StartSpottingEvent {
                        spotter: *attacker,
                        target: *defender,
                        spot_time: attack_stat.current_value(),
                    });
                }
            }
        }
    }
}
