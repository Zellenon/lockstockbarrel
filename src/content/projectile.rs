use crate::{
    graphics::rect,
    twin_stick::{
        physics::GamePhysicsLayer,
        projectile::{projectile, Projectile},
    },
};
use avian2d::prelude::CollisionLayers;
use bevy::prelude::Color;
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree};

pub fn basic_bullet() -> ComponentTree {
    projectile(1., Projectile::default()) + rect(0., 0., 10., 10., Color::srgb(0., 0., 0.))
}

pub fn standard_player_bullet_collision() -> ComponentTree {
    CollisionLayers::new(
        GamePhysicsLayer::Bullet,
        [
            GamePhysicsLayer::Enemy,
            GamePhysicsLayer::MapSolid,
            GamePhysicsLayer::MapDynamic,
        ],
    )
    .store()
}
