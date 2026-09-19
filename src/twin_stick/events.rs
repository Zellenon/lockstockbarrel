use bevy::{ecs::{entity::Entity, message::Message}, math::Vec2, reflect::Reflect};

#[derive(Message, Clone, Copy, PartialEq, Reflect, Debug)]
pub struct AttackMessage {
    pub attacker: Entity,
    pub weapon: Entity,
    pub defender: Entity,
    pub location: Vec2,
    pub direction: Vec2,
}

#[derive(Message, Clone, Copy, PartialEq, Reflect, Debug)]
pub struct KnockbackMessage {
    pub entity: Entity,
    pub direction: Vec2,
    pub force: f32,
}

#[derive(Message, Clone, Copy, PartialEq, Reflect, Debug)]
pub struct DamageMessage {
    pub target: Entity,
    pub source: Entity,
    pub amount: f32,
}
