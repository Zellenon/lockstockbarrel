use bevy::prelude::*;

pub trait Stat {
    fn new(max: f32) -> Self;
    fn max_value(&self) -> f32;
    fn current_value(&self) -> f32;
    fn can_be_negative() -> bool;
}

#[derive(Component)]
pub struct Speed {
    pub current: f32,
    pub max: f32,
}

impl Stat for Speed {
    fn new(max: f32) -> Self {
        Speed { current: max, max }
    }

    fn max_value(&self) -> f32 {
        self.max
    }

    fn current_value(&self) -> f32 {
        self.current
    }

    fn can_be_negative() -> bool {
        false
    }
}
