use bevy::prelude::*;
use std::marker::PhantomData;

use crate::statmod::{ModStyle, MultiplierStyle};

pub trait RPGStat: 'static + Send + Sync {
    fn can_negative() -> bool {
        false
    }

    fn modstyle() -> ModStyle {
        ModStyle::AddMul
    }

    fn multiplier_style() -> MultiplierStyle {
        MultiplierStyle::Additive
    }
}

pub trait RPGResource: RPGStat {
    fn can_overmax() -> bool {
        false
    }
}

#[derive(Component)]
pub struct Stat<T>
where
    T: RPGStat,
{
    pub base: f32,
    pub current: f32,
    pub _phantom: PhantomData<T>,
}

impl<T> Stat<T>
where
    T: RPGStat,
{
    pub fn new(value: f32) -> Self {
        Self {
            base: value,
            current: value,
            _phantom: PhantomData,
        }
    }
    pub fn current_value(&self) -> f32 {
        self.current
    }
}

#[derive(Component, Reflect)]
pub struct Resource<T>
where
    T: RPGStat,
{
    pub max: f32,
    pub current: f32,
    pub _phantom: PhantomData<T>,
}
