use bevy::{
    app::Plugin,
    asset::Handle,
    ecs::system::IntoObserverSystem,
    prelude::{Bundle, Commands, Component, Event, Image, OnAdd, Query, Res, Trigger},
};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree};
use std::sync::Arc;

use crate::assets::images::ImageResources;

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

pub trait ImageFn: 'static + Sync + Send + Fn(&ImageResources) -> Handle<Image> {}

impl<
        T: Fn(&ImageResources) -> bevy::prelude::Handle<bevy::prelude::Image> + Send + Sync + 'static,
    > ImageFn for T
{
}

#[derive(Component, Clone)]
pub struct GiveMeImage(pub Arc<dyn ImageFn>);

pub fn image(image: impl ImageFn) -> ComponentTree {
    GiveMeImage(Arc::new(image)).store()
}

pub struct UtilPlugin;

impl Plugin for UtilPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.observe(give_images);
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
        .insert(requests.get(entity).unwrap().0(&images))
        .remove::<GiveMeImage>();
}
