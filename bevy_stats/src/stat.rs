use std::marker::PhantomData;

use bevy::{
    prelude::{Component, Entity},
    reflect::Reflect,
};

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
