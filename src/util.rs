use bevy::{
    app::Plugin,
    asset::{Asset, Handle},
    ecs::system::{IntoObserverSystem, Resource},
    prelude::{Bundle, Commands, Component, Event, Image, OnAdd, Query, Res, Trigger},
    sprite::Sprite,
};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree};
use std::sync::Arc;

use crate::assets::images::ImageResources;

/// Add the provided function to all entities with component T.
pub fn add_observer_to_component<T, S, E, B, M>(
    observer_function: S,
) -> impl FnMut(Trigger<OnAdd, T>, Commands) -> ()
where
    T: Component,
    B: Bundle,
    E: Event + 'static,
    S: IntoObserverSystem<E, B, M> + Send + Sync + Clone,
{
    move |trigger: Trigger<OnAdd, T>, mut commands: Commands| {
        commands
            .entity(trigger.entity())
            .observe(observer_function.clone());
    }
}

pub trait GimmieFn<T, U>: 'static + Sync + Send + Fn(&ImageResources) -> Handle<T>
where
    T: Asset,
    U: Resource,
{
}

impl<T: Fn(&ImageResources) -> Handle<Image> + Send + Sync + 'static>
    GimmieFn<Image, ImageResources> for T
{
}

#[derive(Component, Clone)]
pub struct GiveMeImage(pub Arc<dyn GimmieFn<Image, ImageResources>>);

pub fn image(image: impl GimmieFn<Image, ImageResources>) -> ComponentTree {
    GiveMeImage(Arc::new(image)).store()
}

pub struct UtilPlugin;

impl Plugin for UtilPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_observer(give_images);
    }
}

pub fn give_images(
    trigger: Trigger<OnAdd, GiveMeImage>,
    images: Res<ImageResources>,
    requests: Query<&GiveMeImage>,
    mut commands: Commands,
) {
    let entity = trigger.entity();
    commands
        .get_entity(entity)
        .unwrap()
        .insert(Sprite {
            image: requests.get(entity).unwrap().0(&images),
            ..Default::default()
        })
        .remove::<GiveMeImage>();
}
