use std::marker::PhantomData;

use bevy::prelude::{Component, Entity, Event};

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

    pub fn offset(value: f32) -> Self {
        Self {
            mod_type: ModType::Offset,
            value,
            _phantom: PhantomData,
        }
    }

    pub fn multiplier(value: f32) -> Self {
        Self {
            mod_type: ModType::Multiplier,
            value,
            _phantom: PhantomData,
        }
    }
    pub fn apply(&self, stat: f32) -> f32 {
        match T::can_negative() {
            true => match self.mod_type {
                ModType::Offset => stat + self.value,
                ModType::Multiplier => stat * self.value,
            },
            false => match self.mod_type {
                ModType::Offset => (stat + self.value).max(0.),
                ModType::Multiplier => (stat * self.value).max(0.),
            },
        }
    }
}

#[derive(Component)]
pub struct StatModifier;

#[derive(Event)]
pub struct DeleteStatMod(pub Entity);

#[derive(Event)]
pub struct StatChangeEvent<T>
where
    T: RPGStat,
{
    pub change: StatValueChange<T>,
    pub target: Entity,
}

#[derive(Event)]
pub struct ResourceChangeEvent<T>
where
    T: RPGResource,
{
    pub change: StatValueChange<T>,
    pub target: Entity,
}
