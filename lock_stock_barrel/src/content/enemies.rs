use std::sync::Arc;

use bevy::{
    ecs::system::EntityCommands,
    prelude::{Handle, Image, Transform, Vec2},
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

pub fn basic_head(head_tex: Handle<Image>) -> ComponentTree {
    let tex = head_tex.clone();
    let func = move |parent: &mut EntityCommands| {
        parent.insert((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Vec2::new(40., 40.).into(),
                    ..Default::default()
                },
                texture: tex.clone(),
                ..Default::default()
            },
            Tracking(None),
        ));
    };
    (Arc::new(func) as EntityCommandSet).into()
}

pub fn basic_legs(leg_tex: Handle<Image>) -> ComponentTree {
    let tex = leg_tex.clone();
    let func = move |parent: &mut EntityCommands| {
        parent.insert((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Vec2::new(30., 35.).into(),
                    ..Default::default()
                },
                transform: Transform::from_xyz(0., 0., -1.),
                texture: tex.clone(),
                ..Default::default()
            },
            Tracking(None),
            Legs::default(),
        ));
    };
    (Arc::new(func) as EntityCommandSet).into()
}

pub fn basic_walker(head_tex: Handle<Image>, leg_tex: Handle<Image>) -> ComponentTree {
    basic_enemy() << basic_legs(leg_tex) << basic_head(head_tex)
}
