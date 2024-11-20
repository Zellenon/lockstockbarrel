use bevy::{color::palettes::css::GREEN, math::Vec3, prelude::Gizmos};

pub(super) fn grid_system(mut gizmos: Gizmos) {
    for x in -100..100 {
        for y in -100..100 {
            gizmos.line(
                Vec3::new(x as f32, y as f32, 0.),
                Vec3::new(x as f32, y as f32, 0.),
                GREEN,
            );
        }
    }
}
