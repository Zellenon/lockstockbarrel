use bevy::{
    app::{App, FixedUpdate, Plugin},
    ecs::{
        component::Component,
        entity::Entity,
        query::{Or, With},
        schedule::{IntoSystemConfigs, SystemSet},
        system::{Commands, Query},
    },
    prelude::IntoSystemSetConfigs,
    reflect::Reflect,
    render::view::Visibility,
    time::Timer,
    utils::{HashMap, HashSet},
};
use display::{display_los, display_tracks};
use identify::{always_identify_tracked, identify_los};
use los::update_los;
use spotting::{do_los_spotting, remove_expired_spots, tick_spotting};
use tracking::{always_track_allies, magic_tracking};

use crate::twin_stick::{actors::Actor, map::Prop, player::Player};

pub mod display;
pub mod identify;
pub mod los;
pub mod spotting;
pub mod tracking;

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
