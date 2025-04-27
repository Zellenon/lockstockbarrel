use core::f32;

use bevy::{
    app::{App, Update},
    color::{
        palettes::css::{GREY, LIME, WHITE, YELLOW},
        Alpha,
    },
    ecs::{
        entity::Entity,
        query::With,
        system::{Query, ResMut},
    },
    gizmos::gizmos::Gizmos,
    math::{Vec2, Vec3Swizzles},
    transform::components::Transform,
};
use bevy_turborand::{DelegatedRng, GlobalRng};

use super::{Identifying, Revealed, Spotting, Tracking, VisionObjects, LOS};
use crate::twin_stick::player::Player;

pub fn display_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            display_los,
            display_tracks,
            display_spotting,
            display_identification,
        ),
    );
}

pub fn display_los(
    player: Query<(Entity, &LOS), With<Player>>,
    mut gizmos: Gizmos,
    vis_obj: Query<&Transform, VisionObjects>,
) {
    if let Ok((e, LOS(los))) = player.get_single() {
        for seen in los.iter().filter(|w| **w != e) {
            if let Ok(pos) = vis_obj.get(*seen) {
                gizmos.circle_2d(pos.translation.xy(), 30., WHITE.with_alpha(0.1));
            }
        }
    }
}

pub fn display_spotting(
    player: Query<(Entity, &Spotting, &Identifying), With<Player>>,
    mut gizmos: Gizmos,
    vis_obj: Query<&Transform, VisionObjects>,
    mut rng: ResMut<GlobalRng>,
) {
    let pos_offset = Vec2::new(rng.f32_normalized(), rng.f32_normalized()) * 0.5;
    let size_offset = rng.f32_normalized() * 0.5;
    let alpha = rng.f32() * 0.7 + 0.3;
    if let Ok((e, Spotting(spots), Identifying(identities))) = player.get_single() {
        for seen in spots
            .iter()
            .filter(|(k, _v)| **k != e && *identities.get(*k).unwrap_or(&0.) < 100.)
        {
            if let Ok(pos) = vis_obj.get(*seen.0) {
                gizmos.rect_2d(
                    pos.translation.xy() + pos_offset,
                    Vec2::splat(20. + size_offset),
                    YELLOW.with_alpha(alpha),
                );
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
                gizmos.rect_2d(pos.translation.xy(), Vec2::splat(30.), LIME.with_alpha(0.5));
            }
        }
    }
}

pub fn display_identification(
    player: Query<(Entity, &Identifying), With<Player>>,
    mut gizmos: Gizmos,
    vis_obj: Query<(&Transform, Option<&Revealed>), VisionObjects>,
) {
    if let Ok((e, Identifying(identities))) = player.get_single() {
        for (id, progress) in identities
            .iter()
            .filter(|(id, progress)| **id != e && **progress < 100.)
        {
            if let Ok((pos, None)) = vis_obj.get(*id) {
                gizmos.arc_2d(
                    pos.translation.xy(),
                    f32::consts::PI * 2. * (progress / 100.),
                    15.,
                    GREY.with_alpha(0.5),
                );
            }
        }
    }
}
