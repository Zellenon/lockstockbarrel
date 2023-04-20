use bevy::{
    ecs::query::ReadOnlyWorldQuery,
    prelude::{App, Entity, Query, With},
};

use crate::{
    do_stat_change,
    statmod::{ModType, StatModifier, StatValueChange},
    RPGStat, Stat, StatChangeEvent,
};

pub trait StatRegisterable {
    fn register_stat<T: RPGStat>(&mut self);
}

impl StatRegisterable for App {
    fn register_stat<T: RPGStat>(&mut self) {
        self.add_event::<StatChangeEvent<T>>();

        self.add_system(do_stat_change::<T>);

        match T::modstyle() {
            crate::stat::ModStyle::AddMul => {
                self.add_system(update_modded_stats_addmul::<T>);
            }
            crate::stat::ModStyle::MulAdd => {
                self.add_system(update_modded_stats_muladd::<T>);
            }

            crate::stat::ModStyle::AverageDifferences => {
                self.add_system(update_modded_stats_avediff::<T>);
            }
            crate::stat::ModStyle::SumDifferences => {
                self.add_system(update_modded_stats_sumdiff::<T>);
            }
        }
    }
}

fn mul_stats<T: RPGStat, F: ReadOnlyWorldQuery>(
    base: f32,
    statlist: &Vec<Entity>,
    mods: &Query<&StatValueChange<T>, F>,
) -> f32 {
    statlist
        .iter()
        .filter_map(|w| mods.get(*w).ok())
        .filter(|w| w.mod_type == ModType::Multiplier)
        .fold(base, |w, v| w * v.value) // TODO: Add handling for additive multiplierstyle
}

fn mul_diff<T: RPGStat, F: ReadOnlyWorldQuery>(
    base: f32,
    statlist: &Vec<Entity>,
    mods: &Query<&StatValueChange<T>, F>,
) -> f32 {
    statlist
        .iter()
        .filter_map(|w| mods.get(*w).ok())
        .filter(|w| w.mod_type == ModType::Multiplier)
        .fold(0., |w, v| w + v.value * base - base) // TODO: Add handling for additive multiplierstyle
}

fn add_stats<T: RPGStat, F: ReadOnlyWorldQuery>(
    base: f32,
    statlist: &Vec<Entity>,
    mods: &Query<&StatValueChange<T>, F>,
) -> f32 {
    statlist
        .iter()
        .filter_map(|w| mods.get(*w).ok())
        .filter(|w| w.mod_type == ModType::Offset)
        .fold(base, |w, v| w + v.value)
}

pub fn update_modded_stats_addmul<T: RPGStat>(
    mut stats: Query<&mut Stat<T>>,
    mods: Query<&StatValueChange<T>, With<StatModifier>>,
) {
    for mut stat in stats.iter_mut() {
        // TODO: Remove dead modifiers
        stat.current = mul_stats(add_stats(stat.base, &stat.mods, &mods), &stat.mods, &mods);
    }
}

pub fn update_modded_stats_muladd<T: RPGStat>(
    mut stats: Query<&mut Stat<T>>,
    mods: Query<&StatValueChange<T>, With<StatModifier>>,
) {
    for mut stat in stats.iter_mut() {
        // TODO: Remove dead modifiers
        stat.current = add_stats(mul_stats(stat.base, &stat.mods, &mods), &stat.mods, &mods);
    }
}

pub fn update_modded_stats_sumdiff<T: RPGStat>(
    mut stats: Query<&mut Stat<T>>,
    mods: Query<&StatValueChange<T>, With<StatModifier>>,
) {
    for mut stat in stats.iter_mut() {
        // TODO: Remove dead modifiers
        stat.current =
            add_stats(stat.base, &stat.mods, &mods) + mul_diff(stat.base, &stat.mods, &mods);
    }
}

pub fn update_modded_stats_avediff<T: RPGStat>(
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
