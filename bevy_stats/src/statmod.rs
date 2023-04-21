use std::marker::PhantomData;

use bevy::prelude::Component;

use crate::{RPGResource, RPGStat};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum ModType {
    Offset,
    Multiplier,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum ModStyle {
    AddMul,
    MulAdd,
    AverageDifferences,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum MultiplierStyle {
    Additive,
    Multiplicative,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum ResourceModUpdateStyle {
    ScaleBoth,
    ScaleOnBuff,
    ScaleOnDebuff,
    NoScale,
}

#[derive(Component)]
pub struct StatValueChange<T>
where
    T: RPGStat,
{
    pub mod_type: ModType,
    pub value: f32,
    _phantom: PhantomData<T>,
}

impl<T> StatValueChange<T>
where
    T: RPGStat,
{
    pub fn new(mod_type: ModType, value: f32) -> Self {
        Self {
            mod_type,
            value,
            _phantom: PhantomData,
        }
    }
}

#[derive(Component)]
pub struct ResourceChange<T>
where
    T: RPGResource,
{
    category: ModType,
    value: f32,
    _phantom: PhantomData<T>,
}

#[derive(Component)]
pub struct StatModifier;
