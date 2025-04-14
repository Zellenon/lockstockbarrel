use bevy::prelude::Color;
use bevy_composable::tree::ComponentTree;

use crate::{graphics::rect, twin_stick::projectile::{projectile, Projectile}};

pub fn basic_bullet() -> ComponentTree {
    projectile(1., Projectile::default()) +
    rect(0., 0., 10., 10., Color::srgb(0., 0., 0.))
}
