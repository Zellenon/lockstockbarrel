use bevy::{prelude::Vec2, window::Window};
use bevy_mod_transform2d::transform2d::Transform2d;

pub fn screen_to_world(p: Vec2, camera_transform: &Transform2d, window: &Window) -> Vec2 {
    let resolution = Vec2::new(window.width() as f32, window.height() as f32);
    let p_ndc = (p * Vec2 { x: 1., y: -1. }) - (resolution * Vec2 { x: 1., y: -1. }) / 2.0;
    let p_world = camera_transform.scale * p_ndc + camera_transform.translation;

    p_world
}

pub mod macros {
    macro_rules! define_asset_loader {
        (
        $plugin_name: ident,
        $loader_name: ident,
        $asset_type: ident,
        $extensions: expr,
        $($string_name: ident -> $handle_name: ident)*;
        $($string_names: ident -> $handle_names: ident)*;
        $($optional_string_name: ident -> $optional_handle_name: ident)*
    ) => {
            #[derive(Default)]
            pub struct $plugin_name;

            impl Plugin for $plugin_name {
                fn build(&self, app: &mut App) {
                    app.init_asset_loader::<$loader_name>()
                        .add_asset::<$asset_type>()
                        .add_system(verify_reference_assets)
                        .add_system(handle_asset_loaded.in_base_set(CoreSet::PreUpdate));
                }
            }
        };
    }

    macro_rules! statelock {
        (
        ($state_name:expr)
    ) => {
            run_if(in_state($state_name))
        };
        ($state_name:expr, $($v:expr),+) => {run_if(in_state($state_name)).statelock!($($v),+)}
    }

    pub(crate) use define_asset_loader;
    pub(crate) use statelock;
}
