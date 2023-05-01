use bevy::prelude::*;

pub struct Arm {
    hand_offset: Vec2,
}

impl Arm {
    pub fn new(hand_offset: Vec2) -> Self {
        Self { hand_offset }
    }
}

impl Default for Arm {
    fn default() -> Self {
        Self::new(Vec2::ZERO)
    }
}
