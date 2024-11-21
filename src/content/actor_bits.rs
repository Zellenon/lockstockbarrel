use crate::{
    game::stats::{Health, MoveSpeed},
    twin_stick::actors::{ActorBundle, Legs, Tracking},
};
use bevy::{
    prelude::{Handle, Image, Transform, Vec2},
    sprite::{Sprite, SpriteBundle},
};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree, wrappers::name};
use bevy_stats::{Resource, Stat};

pub fn basic_actor() -> ComponentTree {
    (
        ActorBundle::default(),
        Stat::<MoveSpeed>::new(50.),
        Resource::<Health>::new(50.),
    )
        .store()
        + name("actor")
}

pub fn basic_head(head_tex: Handle<Image>) -> ComponentTree {
    let tex = head_tex.clone();
    (
        SpriteBundle {
            sprite: Sprite {
                custom_size: Vec2::new(40., 40.).into(),
                ..Default::default()
            },
            texture: tex.clone(),
            ..Default::default()
        },
        Tracking(None),
    )
        .store()
}

pub fn basic_legs(leg_tex: Handle<Image>) -> ComponentTree {
    let tex = leg_tex.clone();
    (
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
    )
        .store()
}
