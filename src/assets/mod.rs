use bevy::{app::Startup, prelude::{Plugin, Reflect}};
use bevy_asset_loader::asset_collection::AssetCollectionApp;
use images::ImageResources;

pub mod audio;
pub mod images;
//pub mod shapes;

#[derive(Reflect, Clone, Debug, PartialEq)]
pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_collection::<ImageResources>();
        //app.add_systems(Startup, ShapeResources::init);
    }
}
