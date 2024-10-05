use bevy::utils::default;

use bevy::{math::Vec2, prelude::Color, sprite::Sprite};
use bevy_composable::{
    tree::{ComponentTree, EntityCommandSet},
    CT,
};
use bevy_twin_stick::{
    bevy_mod_transform2d::transform2d::Transform2d, transform2d_mods::Sprite2dBundle,
};

pub fn rect(x: f32, y: f32, h: f32, w: f32, color: Color) -> ComponentTree {
    CT!(Sprite2dBundle {
        sprite: Sprite {
            color: color,
            custom_size: Some(Vec2::new(w, h)),
            ..default()
        },
        transform: Transform2d::from_xy(x, y),
        ..default()
    })
}
