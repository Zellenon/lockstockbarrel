use std::sync::Arc;

use bevy::{
    ecs::system::EntityCommands,
    prelude::{Handle, Image, Vec2},
    sprite::Sprite,
};
use bevy_composable::tree::{ComponentTree, EntityCommandSet};
use twin_stick::{
    actors::{Legs, Tracking},
    bevy_mod_transform2d::transform2d::Transform2d,
    transform2d_mods::Sprite2dBundle,
};

pub fn basic_head(head_tex: Handle<Image>) -> ComponentTree {
    let tex = head_tex.clone();
    let func = move |parent: &mut EntityCommands| {
        parent.insert((
            Sprite2dBundle {
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
            Sprite2dBundle {
                sprite: Sprite {
                    custom_size: Vec2::new(30., 35.).into(),
                    ..Default::default()
                },
                transform: Transform2d::from_xy(0., 0.).with_z_translation(-1.),
                texture: tex.clone(),
                ..Default::default()
            },
            Tracking(None),
            Legs::default(),
        ));
    };
    (Arc::new(func) as EntityCommandSet).into()
}
