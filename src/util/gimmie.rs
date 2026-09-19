use std::sync::Arc;

use bevy::{
    asset::{Asset, Handle},
    ecs::{
        bundle::Bundle,
        component::Component,
        lifecycle::Add,
        observer::On,
        resource::Resource,
        system::{Commands, Query, Res},
    },
    image::Image,
    sprite::Sprite,
};

use crate::assets::images::ImageResources;

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

pub fn image(image: impl GimmieFn<Image, ImageResources>) -> impl Bundle {
    GiveMeImage(Arc::new(image))
}

pub fn give_images(
    trigger: On<Add, GiveMeImage>,
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
