use bevy::{
    ecs::{entity::Entity, query::With, system::Query},
    transform::components::Transform,
};
use itertools::Itertools;

use super::{Tracking, VisionObjects};
use crate::twin_stick::{
    actors::{Faction, PLAYER_FACTION},
    player::Player,
};

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

pub fn magic_tracking(
    mut query: Query<&mut Tracking>,
    positions: Query<(Entity, &Transform), VisionObjects>,
) {
    positions
        .iter()
        .map(|(e, transform)| (e, transform.translation))
        .permutations(2)
        .map(|w| w.into_iter().collect_tuple().unwrap())
        .filter(|((_e1, t1), (_e2, t2))| t1.distance(*t2) < 150.)
        .for_each(|((e1, _t1), (e2, _t2))| {
            if let Ok(mut track) = query.get_mut(e1) {
                if !track.0.contains(&e2) {
                    track.0.insert(e2);
                }
            }
            if let Ok(mut track) = query.get_mut(e2) {
                if !track.0.contains(&e1) {
                    track.0.insert(e1);
                }
            }
        });
}
