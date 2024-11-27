use crate::{
    assets::images::ImageResources,
    twin_stick::{
        actors::{Faction, MISC_ENEMY_FACTION},
        ai::{tracking::TrackerAI, wander::PerlinWanderAI},
    },
};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree, wrappers::name};

use super::actor_bits::{basic_actor, basic_walker};

pub fn stumbler() -> ComponentTree {
    basic_walker(
        ImageResources::placeholder_head,
        ImageResources::placeholder_legs,
    ) + (
        Faction(MISC_ENEMY_FACTION),
        TrackerAI { precision: 0.8 },
        PerlinWanderAI::new(0.2, 0.8, 0.1, 0.95),
    )
        .store()
        + name("stumbler")
}
