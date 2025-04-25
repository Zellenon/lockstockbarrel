use bevy::{ecs::component::Component, reflect::Reflect};

#[derive(Component, Reflect, Clone, Copy, Hash, Debug)]
pub struct Prop;
