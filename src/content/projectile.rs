use bevy::{prelude::Name, render::color::Color, sprite::Mesh2dHandle};
use bevy_composable::{
    tree::{ComponentTree, EntityCommandSet},
    CT,
};
use bevy_twin_stick::{
    actors::Tracking, bevy_rapier2d::prelude::Ccd, projectile::Lifespan,
    projectile::ProjectileBundle,
};

use crate::graphics::rect;

pub fn basic_bullet() -> ComponentTree {
    return rect(0., 0., 10., 10., Color::rgb(0., 0., 0.))
        + CT!(
            ProjectileBundle::default(),
            Ccd::enabled(),
            Mesh2dHandle::default(),
            Name::new("Bullet"),
            Lifespan::default(),
            Tracking(None)
        );
}
