use crate::twin_stick::{
    physics::GamePhysicsLayer,
    projectile::{projectile, Projectile},
};
use avian2d::prelude::CollisionLayers;
use bevy::ecs::bundle::Bundle;

pub fn basic_bullet() -> impl Bundle {
    projectile(1., Projectile::default())
}

pub fn standard_player_bullet_collision() -> impl Bundle {
    CollisionLayers::new(
        GamePhysicsLayer::Bullet,
        [
            GamePhysicsLayer::Enemy,
            GamePhysicsLayer::MapSolid,
            GamePhysicsLayer::MapDynamic,
        ],
    )
}
