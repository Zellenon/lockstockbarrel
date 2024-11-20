use bevy::{
    asset::Handle,
    ecs::system::Resource,
    prelude::{Image, Reflect},
};
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource, Reflect, Clone, Debug, PartialEq)]
pub struct ImageResources {
    #[asset(path = "img/placeholder_head.png")]
    pub placeholder_head: Handle<Image>,
    #[asset(path = "img/placeholder_legs.png")]
    pub placeholder_legs: Handle<Image>,
    #[asset(path = "img/player_head.png")]
    pub player_head: Handle<Image>,
    #[asset(path = "img/player_legs.png")]
    pub player_legs: Handle<Image>,
}
