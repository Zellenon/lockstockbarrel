use crate::{
    assets::images::ImageResources,
    game::stats::{Damage, Health},
    twin_stick::{
        actors::{Faction, MISC_ENEMY_FACTION},
        ai::{tracking::TrackerAI, wander::PerlinWanderAI},
        physics::GamePhysicsLayer as GPL,
    },
};
use avian2d::prelude::CollisionLayers;
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree, wrappers::name};
use bevy_stats::{Resource, Stat};

use super::actor_bits::basic_walker;

pub fn stumbler() -> ComponentTree {
    basic_walker(
        ImageResources::placeholder_head,
        ImageResources::placeholder_legs,
    ) + (
        Faction(MISC_ENEMY_FACTION),
        CollisionLayers::new(
            GPL::Enemy,
            [
                GPL::Player,
                GPL::Enemy,
                GPL::MapDynamic,
                GPL::MapSolid,
                GPL::Bullet,
            ],
        ),
        Resource::<Health>::new(3.),
        Stat::<Damage>::new(2.),
        TrackerAI { precision: 0.8 },
        PerlinWanderAI::new(0.2, 0.8, 0.1, 0.95),
    )
        .store()
        + name("stumbler")
}
