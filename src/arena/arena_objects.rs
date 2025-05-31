use avian2d::prelude::{Collider, CollisionLayers, RigidBody};
use bevy::{color::Color, core::Name};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree};

use crate::{graphics::rect, twin_stick::physics::GamePhysicsLayer as GPL};

pub fn wall(x: f32, y: f32, width: f32, height: f32) -> ComponentTree {
    rect(x, y, width, height, Color::srgb(0.25, 0.25, 0.75))
        + (
            RigidBody::Static,
            Collider::rectangle(width, height),
            Name::new("Wall"),
            CollisionLayers::new(
                GPL::MapSolid,
                [
                    GPL::Player,
                    GPL::Enemy,
                    GPL::MapDynamic,
                    GPL::MapSolid,
                    GPL::Bullet,
                ],
            ),
        )
            .store()
}
