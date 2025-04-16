use bevy::{
    math::Vec2,
    prelude::{Color, Transform},
    sprite::Sprite,
    utils::default,
};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree};

pub fn rect(x: f32, y: f32, h: f32, w: f32, color: Color) -> ComponentTree {
    (
        Sprite {
            color,
            custom_size: Some(Vec2::new(w, h)),
            ..default()
        },
        Transform::from_xyz(x, y, 0.),
    )
        .store()
}
