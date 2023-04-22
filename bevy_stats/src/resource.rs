use std::marker::PhantomData;

use bevy::{prelude::Component, reflect::Reflect};

use crate::RPGStat;

pub enum ResourceModScaleStyle {
    SumChange,
    Percentage,
    NoScale,
}

pub trait RPGResource: RPGStat {
    fn can_overmax() -> bool {
        false
    }

    fn increase_scaling() -> ResourceModScaleStyle {
        ResourceModScaleStyle::NoScale
    }

    fn decrease_scaling() -> ResourceModScaleStyle {
        ResourceModScaleStyle::NoScale
    }
}

#[derive(Component, Reflect)]
pub struct Resource<T> {
    pub max_base: f32,
    pub max_current: f32,
    pub value_base: f32,
    pub value_current: f32,
    pub _phantom: PhantomData<T>,
}

impl<T> Resource<T>
where
    T: RPGResource,
{
    pub fn new(value: f32) -> Self {
        Self {
            max_base: value,
            max_current: value,
            value_base: value,
            value_current: value,
            _phantom: PhantomData,
        }
    }
    pub fn current_value(&self) -> f32 {
        self.value_current
    }
}
