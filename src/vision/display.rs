use bevy::{
    color::{
        palettes::css::{LIME, YELLOW},
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

use crate::twin_stick::player::Player;

use super::{Tracking, VisionObjects, LOS};

pub fn display_los(
    player: Query<(Entity, &LOS), With<Player>>,
    mut gizmos: Gizmos,
    vis_obj: Query<&Transform, VisionObjects>,
    mut rng: ResMut<GlobalRng>,
) {
    let pos_offset = Vec2::new(rng.f32_normalized(), rng.f32_normalized()) * 0.5;
    let size_offset = rng.f32_normalized() * 0.5;
    let alpha = rng.f32() * 0.7 + 0.3;
    if let Ok((e, LOS(los))) = player.get_single() {
        for seen in los.iter().filter(|w| **w != e) {
            if let Ok(pos) = vis_obj.get(*seen) {
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
