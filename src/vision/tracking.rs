use bevy::{
    app::{App, FixedUpdate},
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::With,
        schedule::IntoSystemConfigs,
        system::Query,
    },
    reflect::Reflect,
    utils::HashSet,
};

use super::{VisionObjects, VisionSystems};
use crate::twin_stick::{
    actors::{Faction, PLAYER_FACTION},
    events::AttackEvent,
    player::Player,
};

#[derive(Component, Default, Reflect, Clone, Debug)]
pub struct Tracking(pub HashSet<Entity>);

#[derive(Event, Clone, Copy, PartialEq, Reflect, Debug)]
pub struct TrackEvent {
    pub tracker: Entity,
    pub target: Entity,
}

#[derive(Event, Clone, Copy, PartialEq, Reflect, Debug)]
pub struct NewTrackEvent {
    pub tracker: Entity,
    pub target: Entity,
}

#[derive(Component, Reflect, Clone, Copy, PartialEq, Debug)]
pub struct TrackAttack;

pub fn track_plugin(app: &mut App) {
    app.register_type::<Tracking>()
        .register_type::<TrackEvent>()
        .register_type::<NewTrackEvent>()
        .add_event::<TrackEvent>()
        .add_event::<NewTrackEvent>();

    app.add_systems(
        FixedUpdate,
        (
            always_track_allies,
            (do_track_attacks, process_track_events).chain(),
        )
            .in_set(VisionSystems::SpotTrack),
    );
}

pub fn always_track_allies(
    mut player: Query<&mut Tracking, With<Player>>,
    allies: Query<(Entity, &Faction)>,
) {
    if let Ok(mut tracking) = player.get_single_mut() {
        for (entity, _) in allies
            .iter()
            .filter(|(e, faction)| faction.0 == PLAYER_FACTION)
        {
            if !tracking.0.contains(&entity) {
                tracking.0.insert(entity);
            }
        }
    }
}

pub fn do_track_attacks(
    mut attack_events: EventReader<AttackEvent>,
    mut track_events: EventWriter<TrackEvent>,
    trackers: Query<Entity, With<Tracking>>,
    track_attacks: Query<Entity, With<TrackAttack>>,
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
        if let Ok(_) = track_attacks.get(*weapon) {
            if let Ok(_) = vision_objects.get(*defender) {
                if let Ok(_) = trackers.get(*attacker) {
                    track_events.send(TrackEvent {
                        tracker: *attacker,
                        target: *defender,
                    });
                }
            }
        }
    }
}

pub fn process_track_events(
    mut track_events: EventReader<TrackEvent>,
    mut new_events: EventWriter<NewTrackEvent>,
    mut trackers: Query<&mut Tracking>,
) {
    for TrackEvent {
        tracker: tracker_e,
        target,
    } in track_events.read()
    {
        if let Ok(mut tracker) = trackers.get_mut(*tracker_e) {
            if !tracker.0.contains(target) {
                tracker.0.insert(*target);
                new_events.send(NewTrackEvent {
                    tracker: *tracker_e,
                    target: *target,
                });
            }
        }
    }
}
