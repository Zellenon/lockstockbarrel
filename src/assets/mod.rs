use bevy::prelude::{Plugin, Reflect};
use bevy_asset_loader::asset_collection::AssetCollectionApp;
use images::ImageResources;

pub mod audio;
pub mod images;

#[derive(Reflect, Clone, Debug, PartialEq)]
pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_collection::<ImageResources>();
    }
}
