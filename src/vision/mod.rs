use std::time::Duration;

use bevy::{
    app::{App, FixedUpdate, Plugin},
    color::{
        palettes::css::{GREEN, LIME, YELLOW},
        Alpha,
    },
    ecs::{
        component::Component,
        entity::Entity,
        query::{Or, With, Without},
        schedule::{IntoSystemConfigs, SystemSet},
        system::{Commands, Query, Res},
    },
    gizmos::gizmos::Gizmos,
    math::{Vec2, Vec3Swizzles},
    prelude::IntoSystemSetConfigs,
    reflect::Reflect,
    render::view::Visibility,
    time::{Time, Timer, TimerMode},
    transform::components::{GlobalTransform, Transform},
    utils::{hashbrown::hash_set, Entry, HashMap, HashSet},
};
use itertools::Itertools;

use crate::twin_stick::{
    actors::{Actor, Faction, PLAYER_FACTION},
    map::Prop,
    player::Player,
};

#[derive(Component, Default, Reflect, Clone, Debug)]
pub struct LOS(pub HashSet<Entity>);

#[derive(Component, Default, Reflect, Clone, Debug)]
pub struct Identifying(pub HashMap<Entity, f32>);

#[derive(Component, Default, Reflect, Clone, Debug)]
pub struct Spotting(pub HashMap<Entity, Timer>);

#[derive(Component, Default, Reflect, Clone, Debug)]
pub struct Tracking(pub HashSet<Entity>);

#[derive(Component, Reflect, Clone, Copy, Debug)]
pub struct Revealed;

#[derive(SystemSet, Hash, Clone, Copy, Debug, Reflect, PartialEq, Eq)]
pub enum VisionSystems {
    LoS,
    SpotTrack,
    RevealLogic,
}

type VisionObjects = Or<(With<Actor>, With<Prop>)>;

pub struct VisionPlugin;

impl Plugin for VisionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Revealed>()
            .register_type::<Identifying>()
            .register_type::<Spotting>()
            .register_type::<Tracking>();

        app.configure_sets(
            FixedUpdate,
            (
                VisionSystems::LoS.before(VisionSystems::SpotTrack),
                VisionSystems::SpotTrack.before(VisionSystems::RevealLogic),
            ),
        );

        app.add_systems(FixedUpdate, (update_los).in_set(VisionSystems::LoS));

        app.add_systems(
            FixedUpdate,
            (
                always_track_allies,
                always_identify_tracked,
                do_los_spotting,
                identify_los,
                magic_tracking,
                (tick_spotting, remove_expired_spots).chain(),
            )
                .in_set(VisionSystems::SpotTrack),
        );

        app.add_systems(
            FixedUpdate,
            (
                reveal_player_awareness,
                sync_revealed_objects_visible,
                (display_tracks, display_los),
            )
                .in_set(VisionSystems::RevealLogic),
        );
    }
}

pub fn sync_revealed_objects_visible(
    mut query: Query<(&mut Visibility, Option<&Revealed>), VisionObjects>,
) {
    for (mut visibility, maybe_revealed) in
        query.iter_mut().filter(|(visibility, maybe_revealed)| {
            (**visibility == Visibility::Visible) != maybe_revealed.is_some()
        })
    {
        *visibility = if let Some(_) = maybe_revealed {
            Visibility::Visible
        } else {
            Visibility::Hidden
        }
    }
}

pub fn reveal_player_awareness(
    mut commands: Commands,
    player: Query<(&Tracking, &Spotting, &Identifying), With<Player>>,
    objects: Query<(Entity, Option<&Revealed>), VisionObjects>,
) {
    if let Ok((tracking, spotting, identify)) = player.get_single() {
        let should_be_revealed = |e: &Entity| {
            (tracking.0.contains(e) || spotting.0.contains_key(e))
                && *identify.0.get(e).unwrap_or(&0.) >= 100.
        };
        let (revealed, unrevealed): (Vec<_>, Vec<_>) =
            objects.iter().partition(|(_, w)| w.is_some());
        for (e, _) in revealed {
            if !should_be_revealed(&e) {
                commands.get_entity(e).unwrap().remove::<Revealed>();
            }
        }
        for (e, _) in unrevealed {
            if should_be_revealed(&e) {
                commands.get_entity(e).unwrap().insert(Revealed);
            }
        }
    }
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

pub fn tick_spotting(mut query: Query<&mut Spotting>, time: Res<Time>) {
    for mut spots in query.iter_mut() {
        for (_, timer) in spots.0.iter_mut() {
            timer.tick(time.delta());
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

pub fn update_los(
    mut seers: Query<(Entity, &mut LOS)>,
    positions: Query<(Entity, &Transform), VisionObjects>,
) {
    for (e1, mut seer) in seers.iter_mut() {
        let t1 = positions.get(e1).unwrap().1.translation;
        seer.0 = positions
            .iter()
            .map(|(e, transform)| (e, transform.translation))
            .filter(|(_e2, t2)| t1.distance(*t2) < 300.)
            .map(|(e2, _t2)| e2)
            .collect()
    }
}

pub fn display_los(
    player: Query<(Entity, &LOS), With<Player>>,
    mut gizmos: Gizmos,
    vis_obj: Query<&Transform, VisionObjects>,
) {
    if let Ok((e, LOS(los))) = player.get_single() {
        for seen in los.iter().filter(|w| **w != e) {
            if let Ok(pos) = vis_obj.get(*seen) {
                gizmos.rect_2d(pos.translation.xy(), Vec2::splat(20.), YELLOW);
            }
        }
    }
}

pub fn display_tracks(
    player: Query<(Entity, &Tracking), With<Player>>,
    mut gizmos: Gizmos,
    vis_obj: Query<&Transform, VisionObjects>,
) {
    if let Ok((e, Tracking(tracking))) = player.get_single() {
        for tracked in tracking.iter().filter(|w| **w != e) {
            if let Ok(pos) = vis_obj.get(*tracked) {
                gizmos.rect_2d(
                    pos.translation.xy(),
                    Vec2::new(30., 30.),
                    LIME.with_alpha(0.5),
                );
            }
        }
    }
}
