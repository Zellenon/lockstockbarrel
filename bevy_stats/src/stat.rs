use std::marker::PhantomData;

use bevy::{
    prelude::{Component, Entity, Query, With},
    reflect::Reflect,
};

use crate::{
    statmod::{ModStyle, MultiplierStyle, StatModifier, StatValueChange},
    systems::{add_stats, mul_diff, mul_stats},
};

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
    pub(crate) mods: Vec<Entity>,
    #[reflect(ignore)]
    _phantom: PhantomData<T>,
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

pub(crate) fn update_modded_stats_addmul<T: RPGStat>(
    mut stats: Query<&mut Stat<T>>,
    mods: Query<&StatValueChange<T>, With<StatModifier>>,
) {
    for mut stat in stats.iter_mut() {
        // TODO: Remove dead modifiers
        stat.current = mul_stats(add_stats(stat.base, &stat.mods, &mods), &stat.mods, &mods);
    }
}

pub(crate) fn update_modded_stats_muladd<T: RPGStat>(
    mut stats: Query<&mut Stat<T>>,
    mods: Query<&StatValueChange<T>, With<StatModifier>>,
) {
    for mut stat in stats.iter_mut() {
        // TODO: Remove dead modifiers
        stat.current = add_stats(mul_stats(stat.base, &stat.mods, &mods), &stat.mods, &mods);
    }
}

pub(crate) fn update_modded_stats_sumdiff<T: RPGStat>(
    mut stats: Query<&mut Stat<T>>,
    mods: Query<&StatValueChange<T>, With<StatModifier>>,
) {
    for mut stat in stats.iter_mut() {
        // TODO: Remove dead modifiers
        stat.current =
            add_stats(stat.base, &stat.mods, &mods) + mul_diff(stat.base, &stat.mods, &mods);
    }
}

pub(crate) fn update_modded_stats_avediff<T: RPGStat>(
    mut stats: Query<&mut Stat<T>>,
    mods: Query<&StatValueChange<T>, With<StatModifier>>,
) {
    for mut stat in stats.iter_mut() {
        // TODO: Remove dead modifiers
        stat.current = (add_stats(stat.base, &stat.mods, &mods)
            + mul_diff(stat.base, &stat.mods, &mods))
            / stat.mods.len() as f32;
    }
}
