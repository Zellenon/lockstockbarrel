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

pub fn basic_enemy(commands: &mut EntityCommands) {
    commands.insert((
        ActorBundle {
            faction: Faction::FactionID(1),
            transform: Transform::default(),
            ..Default::default()
        },
        TrackerAI,
        Stat::<Speed>::new(500.),
        Stat::<Health>::new(50.),
    ));
}

pub fn basic_enemy_meta() -> EntityCommandSet {
    Arc::new(|commands: &mut EntityCommands| {
        commands.insert((
            ActorBundle {
                faction: Faction::FactionID(1),
                transform: Transform::default(),
                ..Default::default()
            },
            TrackerAI,
            Stat::<Speed>::new(500.),
            Stat::<Health>::new(50.),
        ));
    })
}

pub fn basic_walker() -> ComponentTree {
    // let head = head_img.clone();
    // let legs = leg_img.clone();
    let main: ComponentTree = (Arc::new(|e: &mut EntityCommands| {
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
        .into();
    let main = main
        << (Arc::new(|parent: &mut EntityCommands| {
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
            .into();
    main << (Arc::new(|parent: &mut EntityCommands| {
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

pub fn spawn_enemy(commands: &mut Commands, location: Vec2, asset_server: &Res<AssetServer>) {
    commands
        .spawn((
            ActorBundle {
                faction: Faction::FactionID(1),
                transform: Transform::from_translation(location.extend(0.)),
                ..Default::default()
            },
            TrackerAI,
            Stat::<Speed>::new(500.),
            Stat::<Health>::new(50.),
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Vec2::new(40., 40.).into(),
                        ..Default::default()
                    },
                    texture: asset_server.load("img/placeholder_head.png"),
                    ..Default::default()
                },
                Tracking(None),
            ));
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Vec2::new(30., 35.).into(),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0., 0., -1.),
                    texture: asset_server.load("img/placeholder_legs.png"),
                    ..Default::default()
                },
                Tracking(None),
                Legs::default(),
            ));
        });
}
