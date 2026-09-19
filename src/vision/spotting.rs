use bevy::{
    app::{App, FixedUpdate},
    ecs::{
        component::Component,
        entity::Entity,
        prelude::{Message, MessageReader, MessageWriter},
        query::{Changed, With},
        system::{Query, Res},
    },
    platform::collections::{hash_map::Entry, HashMap},
    reflect::Reflect,
    time::{Time, Timer, TimerMode},
};
use bevy_stats::Stat;

use crate::{game::stats::SpotTime, twin_stick::events::AttackMessage};

use super::{VisionObjects, VisionSystems, LOS};

#[derive(Component, Default, Reflect, Clone, Debug)]
pub struct Spotting(pub HashMap<Entity, Timer>);

#[derive(Message, Clone, Copy, PartialEq, Reflect, Debug)]
pub struct StartSpottingMessage {
    pub spotter: Entity,
    pub target: Entity,
    pub spot_time: f32,
}

pub fn spotting_plugin(app: &mut App) {
    app.register_type::<Spotting>()
        .register_type::<StartSpottingMessage>();
    app.add_message::<StartSpottingMessage>();

    app.add_systems(
        FixedUpdate,
        (
            (
                do_los_spotting,
                (tick_spotting, remove_expired_spots).chain(),
                do_spot_attacks,
            ),
            process_spot_messages,
        )
            .chain()
            .in_set(VisionSystems::SpotTrack),
    );
}

//TODO: There has to be a more efficient way to do this
pub fn do_los_spotting(
    spotters: Query<(Entity, &Stat<SpotTime>, &LOS), Changed<LOS>>,
    mut events: MessageWriter<StartSpottingMessage>,
) {
    for (e, stat, LOS(los)) in spotters.iter() {
        for seen_obj in los.iter() {
            events.send(StartSpottingMessage {
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

pub fn process_spot_messages(
    mut events: MessageReader<StartSpottingMessage>,
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
    mut attack_messages: MessageReader<AttackMessage>,
    mut spot_messages: MessageWriter<StartSpottingMessage>,
    spotters: Query<Entity, With<Spotting>>,
    spot_attacks: Query<(Entity, &Stat<SpotTime>)>,
    vision_objects: Query<Entity, VisionObjects>,
) {
    for AttackMessage {
        attacker,
        weapon,
        defender,
        location,
        direction,
    } in attack_messages.read()
    {
        if let Ok((attack, attack_stat)) = spot_attacks.get(*weapon) {
            if let Ok(_) = vision_objects.get(*defender) {
                if let Ok(_) = spotters.get(*attacker) {
                    spot_messages.send(StartSpottingMessage {
                        spotter: *attacker,
                        target: *defender,
                        spot_time: attack_stat.current_value(),
                    });
                }
            }
        }
    }
}
