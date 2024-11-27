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

impl ImageResources {
    pub fn placeholder_head(&self) -> Handle<Image> {
        self.placeholder_head.clone()
    }
    pub fn placeholder_legs(&self) -> Handle<Image> {
        self.placeholder_legs.clone()
    }
    pub fn player_head(&self) -> Handle<Image> {
        self.player_head.clone()
    }
    pub fn player_legs(&self) -> Handle<Image> {
        self.player_legs.clone()
    }
}
