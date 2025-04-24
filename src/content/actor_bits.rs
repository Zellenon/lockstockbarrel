use crate::{
    assets::images::ImageResources,
    game::stats::{Health, MoveSpeed},
    twin_stick::{
        actors::{Actor, Legs, Tracking},
        physics::GamePhysicsLayer as GPL,
    },
    util::{image, GimmieFn},
};
use avian2d::prelude::{Collider, CollisionLayers, LinearDamping, LockedAxes, Mass, RigidBody};
use bevy::{
    image::Image,
    prelude::{Transform, Vec2},
    render::view::{InheritedVisibility, Visibility},
    sprite::Sprite,
};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree, wrappers::name};
use bevy_stats::{Resource, Stat};

pub fn basic_actor() -> ComponentTree {
    (
        Actor::default(),
        Visibility::Visible,
        Transform::default(),
        RigidBody::Dynamic,
        Mass(100.0),
        LinearDamping(13.),
        Collider::circle(15.),
        LockedAxes::ROTATION_LOCKED,
        InheritedVisibility::default(),
        Stat::<MoveSpeed>::new(70.),
        Resource::<Health>::new(5.),
        CollisionLayers::new(
            GPL::Enemy,
            [GPL::Enemy, GPL::Player, GPL::MapSolid, GPL::MapDynamic],
        ),
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
        Legs::default(),
    )
        .store()
}

pub fn basic_walker(
    head_tex: impl GimmieFn<Image, ImageResources>,
    leg_tex: impl GimmieFn<Image, ImageResources>,
) -> ComponentTree {
    basic_actor() << (basic_legs() + image(leg_tex)) << (basic_head() + image(head_tex))
}
