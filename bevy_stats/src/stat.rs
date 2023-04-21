use bevy::prelude::*;
use std::marker::PhantomData;

pub enum ModStyle {
    AddMul,
    MulAdd,
    AverageDifferences,
    SumDifferences,
}

pub enum MultiplierStyle {
    Additive,
    Multiplicative,
}

pub enum ResourceModScaleStyle {
    SumChange,
    Percentage,
    NoScale,
}

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

    fn increase_scaling() -> ResourceModScaleStyle {
        ResourceModScaleStyle::NoScale
    }

    fn decrease_scaling() -> ResourceModScaleStyle {
        ResourceModScaleStyle::NoScale
    }
}

#[derive(Component, Reflect)]
pub struct Stat<T> {
    pub base: f32,
    pub current: f32,
    _phantom: PhantomData<T>,
    pub(crate) mods: Vec<Entity>,
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
            mods: Vec::new(),
        }
    }

    pub fn current_value(&self) -> f32 {
        self.current
    }

    pub fn add_mod(&mut self, new_mod: Entity) {
        self.mods.insert(0, new_mod);
    }

    pub fn remove_mod(&mut self, dead_mod: Entity) {
        let i = self.mods.iter().position(|w| *w == dead_mod);
        if let Some(i) = i {
            self.mods.remove(i);
        }
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
