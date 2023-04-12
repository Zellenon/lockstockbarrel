use bevy::{prelude::Vec2, window::Window};
use bevy_mod_transform2d::transform2d::Transform2d;

pub fn screen_to_world(p: Vec2, camera_transform: &Transform2d, window: &Window) -> Vec2 {
    let resolution = Vec2::new(window.width() as f32, window.height() as f32);
    let p_ndc = p - resolution / 2.0;
    let p_world = camera_transform.scale * p_ndc + camera_transform.translation;

    p_world
}
