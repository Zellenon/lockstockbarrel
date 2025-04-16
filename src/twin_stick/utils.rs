use avian2d::prelude::ExternalImpulse;
use bevy::{
    prelude::{Transform, Vec2, Vec3Swizzles},
    window::Window,
};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree};

pub fn screen_to_world(p: Vec2, camera_transform: &Transform, window: &Window) -> Vec2 {
    let resolution = Vec2::new(window.width() as f32, window.height() as f32);
    let p_ndc = (p * Vec2 { x: 1., y: -1. }) - (resolution * Vec2 { x: 1., y: -1. }) / 2.0;
    let p_world = camera_transform.scale.xy() * p_ndc + camera_transform.translation.xy();

    p_world
}

pub fn pos(x: f32, y: f32) -> ComponentTree {
    Transform::from_xyz(x, y, 0.).store()
}

pub fn instant_force(x: f32, y: f32) -> ComponentTree {
    (ExternalImpulse::new(Vec2::new(x, y))).store()
}
