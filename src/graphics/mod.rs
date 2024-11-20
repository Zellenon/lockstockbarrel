use bevy::prelude::Transform;
use bevy::sprite::SpriteBundle;
use bevy::utils::default;

use bevy::{math::Vec2, prelude::Color, sprite::Sprite};
use bevy_composable::app_impl::ComponentTreeable;
use bevy_composable::tree::ComponentTree;

pub fn rect(x: f32, y: f32, h: f32, w: f32, color: Color) -> ComponentTree {
    (SpriteBundle {
        sprite: Sprite {
            color,
            custom_size: Some(Vec2::new(w, h)),
            ..default()
        },
        transform: Transform::from_xyz(x, y, 0.),
        ..default()
    })
    .store()
}
