use std::sync::Arc;

use bevy::{
    ecs::system::{Commands, EntityCommands},
    prelude::{AssetServer, BuildChildren, Handle, Image, Res, Transform, Vec2},
    sprite::{Sprite, SpriteBundle},
};
use bevy_composable::*;
use bevy_stats::{Health, Speed, Stat};
use twin_stick::{
    actors::{ActorBundle, Faction, Legs, Tracking},
    ai::TrackerAI,
};

pub fn basic_enemy() -> ComponentTree {
    (Arc::new(|e: &mut EntityCommands| {
        e.insert((
            ActorBundle {
                faction: Faction::FactionID(1),
                ..Default::default()
            },
            TrackerAI,
            Stat::<Speed>::new(500.),
            Stat::<Health>::new(50.),
        ));
    }) as EntityCommandSet)
        .into()
}

pub fn basic_head() -> ComponentTree {
    (Arc::new(|parent: &mut EntityCommands| {
        parent.insert((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Vec2::new(40., 40.).into(),
                    ..Default::default()
                },
                // texture: head,
                ..Default::default()
            },
            Tracking(None),
        ));
    }) as EntityCommandSet)
        .into()
}

pub fn basic_legs() -> ComponentTree {
    (Arc::new(|parent: &mut EntityCommands| {
        parent.insert((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Vec2::new(30., 35.).into(),
                    ..Default::default()
                },
                transform: Transform::from_xyz(0., 0., -1.),
                // texture: legs,
                ..Default::default()
            },
            Tracking(None),
            Legs::default(),
        ));
    }) as EntityCommandSet)
        .into()
}

pub fn basic_walker() -> ComponentTree {
    basic_enemy() << basic_legs() << basic_head()
}
