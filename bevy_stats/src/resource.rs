use std::marker::PhantomData;

use bevy::{
    prelude::{Commands, Component, Entity, Query, Without},
    reflect::Reflect,
};

use crate::{RPGStat, Stat};

pub enum ResourceModScaleStyle {
    SumChange,
    Percentage,
    NoScale,
}

#[derive(Component)]
pub struct InitResourcePercentagePolicy {
    pub percentage: f32,
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
    pub current: f32,
    pub percent: f32,
    #[reflect(ignore)]
    pub _phantom: PhantomData<T>,
}

impl<T> Resource<T>
where
    T: RPGResource,
{
    pub fn new(value: f32) -> Self {
        Self {
            current: value,
            percent: 1.0,
            _phantom: PhantomData,
        }
    }
    pub fn current_value(&self) -> f32 {
        self.current
    }
}

pub(crate) fn ensure_max_stat<T: RPGResource>(
    mut commands: Commands,
    query: Query<(Entity, &Resource<T>), (Without<InitResourcePercentagePolicy>, Without<Stat<T>>)>,
) {
    for (entity, resource) in query.iter() {
        commands
            .get_entity(entity)
            .unwrap()
            .insert(Stat::<T>::new(resource.current_value()));
    }
}

pub(crate) fn ensure_max_stat_with_percentage<T: RPGResource>(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Resource<T>, &InitResourcePercentagePolicy), Without<Stat<T>>>,
) {
    for (entity, mut resource, InitResourcePercentagePolicy { percentage }) in query.iter_mut() {
        commands
            .get_entity(entity)
            .unwrap()
            .insert(Stat::<T>::new(resource.current_value()));
        resource.percent = *percentage;
        resource.current *= *percentage;
    }
}
