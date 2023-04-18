use std::sync::Arc;

use bevy::{
    ecs::system::EntityCommands,
    prelude::{Handle, Image, Name},
};
use bevy_composable::tree::{ComponentTree, EntityCommandSet};
use bevy_stats::{Health, Speed, Stat};
use twin_stick::{
    actors::{ActorBundle, Faction},
    ai::{tracking::TrackerAI, wander::PerlinWanderAI},
};

use super::actor_bits::{basic_head, basic_legs};

pub fn basic_enemy() -> ComponentTree {
    (Arc::new(|e: &mut EntityCommands| {
        e.insert((
            ActorBundle {
                faction: Faction::FactionID(1),
                ..Default::default()
            },
            TrackerAI { precision: 0.8 },
            PerlinWanderAI::new(0.3, 0.8, 0.1, 0.8),
            Stat::<Speed>::new(500.),
            Stat::<Health>::new(50.),
            Name::new("Enemy"),
        ));
    }) as EntityCommandSet)
        .into()
}

pub fn basic_walker(head_tex: Handle<Image>, leg_tex: Handle<Image>) -> ComponentTree {
    basic_enemy() << basic_legs(leg_tex) << basic_head(head_tex)
}
