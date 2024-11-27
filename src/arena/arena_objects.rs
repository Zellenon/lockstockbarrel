use avian2d::prelude::{Collider, RigidBody};
use bevy::{color::Color, core::Name};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree};

use crate::graphics::rect;

pub fn wall(x: f32, y: f32, width: f32, height: f32) -> ComponentTree {
    println!("Spawning wall at {}, {}", x, y);

    rect(x, y, width, height, Color::srgb(0.25, 0.25, 0.75))
        + (
            RigidBody::Static,
            Collider::rectangle(width, height),
            Name::new("Wall"),
        )
            .store()
}
