use crate::{
    game::stats::{Health, MoveSpeed},
    twin_stick::actors::{ActorBundle, Legs, Tracking},
    util::{image, ImageFn},
};
use bevy::{
    prelude::{Transform, Vec2},
    sprite::Sprite,
};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree, wrappers::name};
use bevy_stats::{Resource, Stat};

pub fn basic_actor() -> ComponentTree {
    (
        ActorBundle::default(),
        Stat::<MoveSpeed>::new(70.),
        Resource::<Health>::new(50.),
    )
        .store()
    + name("actor")
}

pub fn basic_head() -> ComponentTree {
    (
        Sprite {
            custom_size: Vec2::new(40., 40.).into(),
            ..Default::default()
        },
        Tracking(None),
    )
        .store()
}

pub fn basic_legs() -> ComponentTree {
    (
        Sprite {
            custom_size: Vec2::new(30., 35.).into(),
            ..Default::default()
        },
        Transform::from_xyz(0., 0., -1.),
        Tracking(None),
        Legs::default()
    )
        .store()
}

pub fn basic_walker(head_tex: impl ImageFn, leg_tex: impl ImageFn) -> ComponentTree {
    basic_actor() << (basic_legs() + image(leg_tex)) << (basic_head() + image(head_tex))
}
