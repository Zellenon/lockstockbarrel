use bevy::{
    app::{App, FixedUpdate},
    ecs::{
        component::Component,
        entity::Entity,
        prelude::{Message, MessageReader, MessageWriter},
        query::With,
        schedule::IntoScheduleConfigs,
        system::Query,
    },
    platform::collections::HashSet,
    prelude::Single,
    reflect::Reflect,
};

use super::{VisionObjects, VisionSystems};
use crate::twin_stick::{
    actors::{Faction, PLAYER_FACTION},
    events::AttackMessage,
    player::Player,
};

#[derive(Component, Default, Reflect, Clone, Debug)]
pub struct Tracking(pub HashSet<Entity>);

#[derive(Message, Clone, Copy, PartialEq, Reflect, Debug)]
pub struct TrackMessage {
    pub tracker: Entity,
    pub target: Entity,
}

#[derive(Message, Clone, Copy, PartialEq, Reflect, Debug)]
pub struct NewTrackMessage {
    pub tracker: Entity,
    pub target: Entity,
}

#[derive(Component, Reflect, Clone, Copy, PartialEq, Debug)]
pub struct TrackAttack;

pub fn track_plugin(app: &mut App) {
    app.register_type::<Tracking>()
        .register_type::<TrackMessage>()
        .register_type::<NewTrackMessage>()
        .add_message::<TrackMessage>()
        .add_message::<NewTrackMessage>();

    app.add_systems(
        FixedUpdate,
        (
            (always_track_allies, do_track_attacks),
            process_track_messages,
        )
            .chain()
            .in_set(VisionSystems::SpotTrack),
    );
}

pub fn always_track_allies(
    mut tracking: Single<&mut Tracking, With<Player>>,
    allies: Query<(Entity, &Faction)>,
) {
    for (entity, _) in allies
        .iter()
        .filter(|(e, faction)| faction.0 == PLAYER_FACTION)
    {
        if !tracking.0.contains(&entity) {
            tracking.0.insert(entity);
        }
    }
}

pub fn do_track_attacks(
    mut attack_messages: MessageReader<AttackMessage>,
    mut track_messages: MessageWriter<TrackMessage>,
    trackers: Query<Entity, With<Tracking>>,
    track_attacks: Query<Entity, With<TrackAttack>>,
    vision_objects: Query<Entity, VisionObjects>,
) {
    for AttackMessage {
        attacker,
        weapon,
        defender,
        location: _,
        direction: _,
    } in attack_messages.read()
    {
        if let Ok(_) = track_attacks.get(*weapon) {
            if let Ok(_) = vision_objects.get(*defender) {
                if let Ok(_) = trackers.get(*attacker) {
                    track_messages.write(TrackMessage {
                        tracker: *attacker,
                        target: *defender,
                    });
                }
            }
        }
    }
}

pub fn process_track_messages(
    mut track_messages: MessageReader<TrackMessage>,
    mut new_messages: MessageWriter<NewTrackMessage>,
    mut trackers: Query<&mut Tracking>,
) {
    for TrackMessage {
        tracker: tracker_e,
        target,
    } in track_messages.read()
    {
        if let Ok(mut tracker) = trackers.get_mut(*tracker_e) {
            if !tracker.0.contains(target) {
                tracker.0.insert(*target);
                new_messages.write(NewTrackMessage {
                    tracker: *tracker_e,
                    target: *target,
                });
            }
        }
    }
}
