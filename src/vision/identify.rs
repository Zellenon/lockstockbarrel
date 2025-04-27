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
    time::Time,
    utils::{Entry, HashMap},
};
use bevy_stats::Stat;

use crate::{game::stats::IdentifyPower, twin_stick::events::AttackEvent};

use super::{
    tracking::{do_track_attacks, process_track_events},
    Tracking, VisionObjects, VisionSystems, LOS,
};

#[derive(Component, Default, Reflect, Clone, Debug)]
pub struct Identifying(pub HashMap<Entity, f32>);

#[derive(Event, Clone, Copy, PartialEq, Reflect, Debug)]
pub struct IdentifyEvent {
    pub identifier: Entity,
    pub target: Entity,
    pub power: f32,
}

pub fn identify_plugin(app: &mut App) {
    app.register_type::<IdentifyEvent>()
        .register_type::<Identifying>();
    app.add_event::<IdentifyEvent>();

    app.add_systems(
        FixedUpdate,
        (
            (
                always_identify_tracked.after(process_track_events),
                identify_los,
                do_identify_attacks,
            ),
            (receive_identify_events).chain(),
        )
            .in_set(VisionSystems::SpotTrack),
    );
}

pub fn always_identify_tracked(
    trackers: Query<(Entity, &Tracking), (Changed<Tracking>, With<Identifying>)>,
    mut events: EventWriter<IdentifyEvent>,
) {
    for (identifier, tracking) in trackers.iter() {
        for target in tracking.0.iter() {
            events.send(IdentifyEvent {
                identifier,
                target: *target,
                power: 100.,
            });
        }
    }
}

pub fn identify_los(
    seers: Query<(Entity, &LOS, &Identifying, &Stat<IdentifyPower>)>,
    mut events: EventWriter<IdentifyEvent>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();
    for (e, los, identifying, stat) in seers.iter() {
        for target in los.0.iter() {
            if identifying.0.get(target).unwrap_or(&0.) < &100. {
                events.send(IdentifyEvent {
                    identifier: e,
                    target: *target,
                    power: delta * stat.current_value(),
                });
            }
        }
    }
}

pub fn receive_identify_events(
    mut ident_events: EventReader<IdentifyEvent>,
    mut identifiers: Query<&mut Identifying>,
) {
    for IdentifyEvent {
        identifier,
        target,
        power,
    } in ident_events.read()
    {
        if let Ok(mut identifier) = identifiers.get_mut(*identifier) {
            match identifier.0.entry(*target) {
                Entry::Occupied(mut occupied_entry) => {
                    let entry = occupied_entry.get();
                    if *entry < 100. {
                        *occupied_entry.get_mut() = entry + power;
                    }
                }
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(*power);
                }
            }
        }
    }
}

pub fn do_identify_attacks(
    mut attack_events: EventReader<AttackEvent>,
    mut spot_events: EventWriter<IdentifyEvent>,
    identifiers: Query<Entity, With<Identifying>>,
    identify_attacks: Query<(Entity, &Stat<IdentifyPower>)>,
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
        if let Ok((attack, attack_stat)) = identify_attacks.get(*weapon) {
            if let Ok(_) = vision_objects.get(*defender) {
                if let Ok(_) = identifiers.get(*attacker) {
                    spot_events.send(IdentifyEvent {
                        identifier: *attacker,
                        target: *defender,
                        power: attack_stat.current_value(),
                    });
                }
            }
        }
    }
}
