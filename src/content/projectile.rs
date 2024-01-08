use bevy::{
    prelude::{Color, Name},
    sprite::Mesh2dHandle,
};
use bevy_composable::{
    tree::{ComponentTree, EntityCommandSet},
    CT,
};
use bevy_twin_stick::{bevy_rapier2d::prelude::Ccd, projectile::ProjectileBundle};

use crate::graphics::Circle;

pub fn basic_bullet() -> ComponentTree {
    return CT!(
        ProjectileBundle::default(),
        Ccd::enabled(),
        Circle::new(5., Color::BLACK),
        Mesh2dHandle::default(),
        Name::new("Bullet")
    );
}
