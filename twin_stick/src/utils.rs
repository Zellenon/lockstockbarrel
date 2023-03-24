use bevy::prelude::*;

pub fn screen_to_world(p: Vec2, camera_transform: &Transform, window: &Window) -> Vec2 {
    let resolution = Vec2::new(window.width() as f32, window.height() as f32);
    let p_ndc = p - resolution / 2.0;
    let p_world = *camera_transform * p_ndc.extend(0.0);

    p_world.truncate()
}

pub fn get_angle(direction: Vec3) -> Quat {
    let direction = Vec3::normalize_or_zero(direction);
    let right = Vec3::new(0., 0., 1.).cross(direction).normalize_or_zero();
    Quat::from_mat3(&Mat3::from_cols(right, direction, Vec3::new(0., 0., -1.)))
}
