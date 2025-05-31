use bevy::{
    ecs::{entity::Entity, event::Event},
    math::Vec2,
    reflect::Reflect,
};

#[derive(Event, Clone, Copy, PartialEq, Reflect, Debug)]
pub struct AttackEvent {
    pub attacker: Entity,
    pub weapon: Entity,
    pub defender: Entity,
    pub location: Vec2,
    pub direction: Vec2,
}

#[derive(Event, Clone, Copy, PartialEq, Reflect, Debug)]
pub struct KnockbackEvent {
    pub entity: Entity,
    pub direction: Vec2,
    pub force: f32,
}

#[derive(Event, Clone, Copy, PartialEq, Reflect, Debug)]
pub struct DamageEvent {
    pub target: Entity,
    pub source: Entity,
    pub amount: f32,
}
