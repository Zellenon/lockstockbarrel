use crate::{
    twin_stick::{
        physics::GamePhysicsLayer,
        projectile::{projectile, Projectile},
    },
    util::spawning::Spawnable,
};
use avian2d::prelude::CollisionLayers;
use bevy::ecs::bundle::Bundle;

pub fn basic_bullet() -> impl Spawnable {
    projectile(1., Projectile::default())
}

pub fn standard_player_bullet_collision() -> impl Spawnable {
    CollisionLayers::new(
        GamePhysicsLayer::Bullet,
        [
            GamePhysicsLayer::Enemy,
            GamePhysicsLayer::MapSolid,
            GamePhysicsLayer::MapDynamic,
        ],
    )
}
