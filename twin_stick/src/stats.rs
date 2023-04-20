use bevy::prelude::Component;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct Knockback(pub f32);
