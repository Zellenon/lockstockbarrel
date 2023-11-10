use bevy::{
    prelude::{Color, Handle, Name, Vec2},
    sprite::Mesh2dHandle,
};
use bevy_composable::{
    tree::{ComponentTree, EntityCommandSet},
    CT,
};
use bevy_prototype_lyon::{
    prelude::{Fill, GeometryBuilder, Stroke},
    render::ShapeMaterial,
    shapes,
};
use bevy_twin_stick::{bevy_rapier2d::prelude::Ccd, projectile::ProjectileBundle};

pub fn basic_bullet() -> ComponentTree {
    return CT!(
        ProjectileBundle::default(),
        GeometryBuilder::build_as(&shapes::Circle {
            radius: 5.,
            center: Vec2::ZERO,
        }),
        Fill::color(Color::YELLOW),
        Stroke::new(Color::BLACK, 2.0),
        Ccd::enabled(),
        Mesh2dHandle::default(),
        Handle::<ShapeMaterial>::default(),
        Name::new("Bullet")
    );
}
