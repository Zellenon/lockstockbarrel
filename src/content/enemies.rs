use crate::twin_stick::{
    actors::Faction,
    ai::{tracking::TrackerAI, wander::PerlinWanderAI},
};
use bevy::prelude::{Handle, Image};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree, wrappers::name};

use super::actor_bits::{basic_actor, basic_head, basic_legs};

pub fn basic_enemy() -> ComponentTree {
    basic_actor()
        + (
            Faction::FactionID(1),
            TrackerAI { precision: 0.8 },
            PerlinWanderAI::new(0.2, 0.8, 0.1, 0.95),
        )
            .store()
        + name("enemy")
}

pub fn basic_walker(head_tex: Handle<Image>, leg_tex: Handle<Image>) -> ComponentTree {
    basic_enemy() << basic_legs(leg_tex) << basic_head(head_tex)
}
