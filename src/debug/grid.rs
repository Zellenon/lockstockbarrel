use bevy::{
    color::{palettes::css::GREEN, Gray, LinearRgba},
    math::{Isometry2d, UVec2, Vec2},
    prelude::Gizmos,
};

pub(super) fn grid_system(mut gizmos: Gizmos) {
    gizmos
        .grid_2d(
            Isometry2d::IDENTITY,
            UVec2::new(100, 100),
            Vec2::new(100., 100.),
            // Dark gray
            LinearRgba::gray(0.05),
        )
        .outer_edges();
}
