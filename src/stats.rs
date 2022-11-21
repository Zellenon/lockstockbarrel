use std::marker::PhantomData;

use bevy::prelude::*;

#[derive(Component)]
pub struct Stat<T>
where
    T: ActorStat,
{
    pub max: f32,
    pub current: f32,
    pub phantom: PhantomData<T>,
}

pub trait ActorStat {
    fn can_negative() -> bool;
    fn can_overmax() -> bool;
}

impl<T> Stat<T>
where
    T: ActorStat,
{
    pub fn new(max: f32) -> Self {
        Self {
            max,
            current: max,
            phantom: PhantomData,
        }
    }
    pub fn max_value(&self) -> f32 {
        self.max
    }
    pub fn current_value(&self) -> f32 {
        self.current
    }
    pub fn can_be_negative() -> bool {
        false
    }
    pub fn can_overmax() -> bool {
        false
    }
}

pub struct Speed;
impl ActorStat for Speed {
    fn can_negative() -> bool {
        false
    }

    fn can_overmax() -> bool {
        true
    }
}

pub struct Health;
impl ActorStat for Health {
    fn can_negative() -> bool {
        true
    }

    fn can_overmax() -> bool {
        false
    }
}
