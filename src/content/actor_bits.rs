use bevy::{
    ecs::children,
    image::Image,
    prelude::{Bundle, Transform, Vec2},
    sprite::Sprite,
};

use crate::{
    assets::images::ImageResources,
    twin_stick::{
        actors::{basic_actor, Legs, Tracking},
        physics::GamePhysicsLayer as GPL,
        player::Player,
    },
    util::gimmie::{image, GimmieFn},
};

pub fn basic_head() -> impl Bundle {
    (Sprite {
        custom_size: Vec2::new(40., 40.).into(),
        ..Default::default()
    },)
}

pub fn basic_legs() -> impl Bundle {
    (
        Sprite {
            custom_size: Vec2::new(30., 35.).into(),
            ..Default::default()
        },
        Legs::default(),
    )
}

pub fn basic_walker(
    head_tex: impl GimmieFn<Image, ImageResources>,
    leg_tex: impl GimmieFn<Image, ImageResources>,
) -> impl Bundle {
    (
        basic_actor(),
        children![
            (basic_legs(), image(leg_tex)),
            (basic_head(), image(head_tex))
        ],
    )
}
