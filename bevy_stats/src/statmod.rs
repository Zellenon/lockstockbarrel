use std::marker::PhantomData;

use bevy::prelude::{Component, Entity};

use crate::{RPGResource, RPGStat};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum ModStyle {
    AddMul,
    MulAdd,
    AverageDifferences,
    SumDifferences,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum ModType {
    Offset,
    Multiplier,
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

    pub fn apply(&self, stat: f32) -> f32 {
        match self.mod_type {
            ModType::Offset => stat + self.value,
            ModType::Multiplier => stat * self.value,
        }
    }
}

#[derive(Component)]
pub struct StatModifier;

pub struct DeleteStatMod(pub Entity);

pub struct StatChangeEvent<T>
where
    T: RPGStat,
{
    pub change: StatValueChange<T>,
    pub target: Entity,
}

pub struct ResourceChangeEvent<T>
where
    T: RPGResource,
{
    pub change: StatValueChange<T>,
    pub target: Entity,
}
