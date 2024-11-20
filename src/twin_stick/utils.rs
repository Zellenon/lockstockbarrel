use bevy::prelude::Vec3Swizzles;
use bevy::{
    prelude::{Transform, Vec2},
    window::Window,
};

pub fn screen_to_world(p: Vec2, camera_transform: &Transform, window: &Window) -> Vec2 {
    let resolution = Vec2::new(window.width() as f32, window.height() as f32);
    let p_ndc = (p * Vec2 { x: 1., y: -1. }) - (resolution * Vec2 { x: 1., y: -1. }) / 2.0;
    let p_world = camera_transform.scale.xy() * p_ndc + camera_transform.translation.xy();

    p_world
}
