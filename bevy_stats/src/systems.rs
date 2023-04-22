use bevy::{
    ecs::query::ReadOnlyWorldQuery,
    prelude::{App, Commands, Entity, EventReader, Query, With},
};

use crate::{
    statmod::{
        DeleteStatMod, ModStyle, ModType, ResourceChangeEvent, StatChangeEvent, StatModifier,
        StatValueChange,
    },
    RPGResource, RPGStat, Resource, Stat,
};

pub trait StatRegisterable {
    fn register_stat<T: RPGStat>(&mut self) -> &mut App;
    fn register_resource<T: RPGResource>(&mut self) -> &mut App;
}

impl StatRegisterable for App {
    fn register_stat<T: RPGStat>(&mut self) -> &mut App {
        // self.register_type::<Stat<T>>();
        self.add_event::<StatChangeEvent<T>>();

        match T::modstyle() {
            ModStyle::AddMul => {
                self.add_system(update_modded_stats_addmul::<T>);
            }
            ModStyle::MulAdd => {
                self.add_system(update_modded_stats_muladd::<T>);
            }
            ModStyle::AverageDifferences => {
                self.add_system(update_modded_stats_avediff::<T>);
            }
            ModStyle::SumDifferences => {
                self.add_system(update_modded_stats_sumdiff::<T>);
            }
        }
        return self;
    }

    fn register_resource<T: RPGResource>(&mut self) -> &mut App {
        self.register_stat::<T>();
        self.add_event::<ResourceChangeEvent<T>>();
        return self;
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
        .fold(base, |w, v| w * (1. + v.value)) // TODO: Add handling for additive multiplierstyle
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
        .fold(0., |w, v| w + v.value * base) // TODO: Add handling for additive multiplierstyle
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

fn update_modded_stats_addmul<T: RPGStat>(
    mut stats: Query<&mut Stat<T>>,
    mods: Query<&StatValueChange<T>, With<StatModifier>>,
) {
    for mut stat in stats.iter_mut() {
        // TODO: Remove dead modifiers
        stat.current = mul_stats(add_stats(stat.base, &stat.mods, &mods), &stat.mods, &mods);
    }
}

fn update_modded_stats_muladd<T: RPGStat>(
    mut stats: Query<&mut Stat<T>>,
    mods: Query<&StatValueChange<T>, With<StatModifier>>,
) {
    for mut stat in stats.iter_mut() {
        // TODO: Remove dead modifiers
        stat.current = add_stats(mul_stats(stat.base, &stat.mods, &mods), &stat.mods, &mods);
    }
}

fn update_modded_stats_sumdiff<T: RPGStat>(
    mut stats: Query<&mut Stat<T>>,
    mods: Query<&StatValueChange<T>, With<StatModifier>>,
) {
    for mut stat in stats.iter_mut() {
        // TODO: Remove dead modifiers
        stat.current =
            add_stats(stat.base, &stat.mods, &mods) + mul_diff(stat.base, &stat.mods, &mods);
    }
}

fn update_modded_stats_avediff<T: RPGStat>(
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

pub fn delete_stat_mod(mut commands: Commands, mut events: EventReader<DeleteStatMod>) {
    for DeleteStatMod(entity) in events.iter() {
        commands.get_entity(*entity).unwrap().despawn();
    }
}

pub fn change_stat<T: RPGStat>(
    mut events: EventReader<StatChangeEvent<T>>,
    mut query: Query<&mut Stat<T>>,
) {
    for StatChangeEvent { change, target } in events.iter() {
        let base = change.apply(query.get(*target).unwrap().base);
        query.get_mut(*target).unwrap().base = base;
    }
}

pub fn change_resource<T: RPGResource>(
    mut events: EventReader<ResourceChangeEvent<T>>,
    mut query: Query<&mut Resource<T>>,
) {
    for ResourceChangeEvent { change, target } in events.iter() {
        let base = change.apply(query.get(*target).unwrap().value_base);
        query.get_mut(*target).unwrap().value_base = base;
    }
}
