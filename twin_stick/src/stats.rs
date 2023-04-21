use bevy::{prelude::Component, reflect::Reflect};

#[derive(Component, Reflect)]
pub struct Speed(pub f32);

#[derive(Component, Reflect)]
pub struct Health(pub f32);

#[derive(Component, Reflect)]
pub struct Knockback(pub f32);
