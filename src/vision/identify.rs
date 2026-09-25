use bevy::{
    app::{App, FixedUpdate},
    ecs::schedule::IntoScheduleConfigs,
    ecs::{
        component::Component,
        entity::Entity,
        message::{Message, MessageReader, MessageWriter},
        query::{Changed, With},
        system::{Query, Res},
    },
    platform::collections::{hash_map::Entry, HashMap},
    reflect::Reflect,
    time::Time,
};
use bevy_stats::Stat;

use crate::{game::stats::IdentifyPower, twin_stick::events::AttackMessage};

use super::{
    tracking::{do_track_attacks, process_track_messages},
    Tracking, VisionObjects, VisionSystems, LOS,
};

#[derive(Component, Default, Reflect, Clone, Debug)]
pub struct Identifying(pub HashMap<Entity, f32>);

#[derive(Message, Clone, Copy, PartialEq, Reflect, Debug)]
pub struct IdentifyMessage {
    pub identifier: Entity,
    pub target: Entity,
    pub power: f32,
}

pub fn identify_plugin(app: &mut App) {
    app.register_type::<IdentifyMessage>()
        .register_type::<Identifying>();
    app.add_message::<IdentifyMessage>();

    app.add_systems(
        FixedUpdate,
        (
            (
                always_identify_tracked.after(process_track_messages),
                identify_los,
                do_identify_attacks,
            ),
            (receive_identify_messages).chain(),
        )
            .in_set(VisionSystems::SpotTrack),
    );
}

pub fn always_identify_tracked(
    trackers: Query<(Entity, &Tracking), (Changed<Tracking>, With<Identifying>)>,
    mut events: MessageWriter<IdentifyMessage>,
) {
    for (identifier, tracking) in trackers.iter() {
        for target in tracking.0.iter() {
            events.write(IdentifyMessage {
                identifier,
                target: *target,
                power: 100.,
            });
        }
    }
}

pub fn identify_los(
    seers: Query<(Entity, &LOS, &Identifying, &Stat<IdentifyPower>)>,
    mut events: MessageWriter<IdentifyMessage>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();
    for (e, los, identifying, stat) in seers.iter() {
        for target in los.0.iter() {
            if identifying.0.get(target).unwrap_or(&0.) < &100. {
                events.write(IdentifyMessage {
                    identifier: e,
                    target: *target,
                    power: delta * stat.current_value(),
                });
            }
        }
    }
}

pub fn receive_identify_messages(
    mut ident_messages: MessageReader<IdentifyMessage>,
    mut identifiers: Query<&mut Identifying>,
) {
    for IdentifyMessage {
        identifier,
        target,
        power,
    } in ident_messages.read()
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
    mut attack_messages: MessageReader<AttackMessage>,
    mut spot_messages: MessageWriter<IdentifyMessage>,
    identifiers: Query<Entity, With<Identifying>>,
    identify_attacks: Query<(Entity, &Stat<IdentifyPower>)>,
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
        if let Ok((attack, attack_stat)) = identify_attacks.get(*weapon) {
            if let Ok(_) = vision_objects.get(*defender) {
                if let Ok(_) = identifiers.get(*attacker) {
                    spot_messages.write(IdentifyMessage {
                        identifier: *attacker,
                        target: *defender,
                        power: attack_stat.current_value(),
                    });
                }
            }
        }
    }
}
