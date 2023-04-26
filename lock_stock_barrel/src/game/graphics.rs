use bevy::prelude::*;
use bevy_vfx_bag::post_processing::masks::Mask;
use bevy_vfx_bag::post_processing::pixelate::Pixelate;
use bevy_vfx_bag::post_processing::wave::Wave;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_vfx_bag::BevyVfxBagPlugin::default());
        app.add_system(setup_camera);
        app.add_system(tick_camera_effects);
    }
}

pub fn setup_camera(
    cameras: Query<Entity, (With<Camera2d>, Without<Wave>, Without<Camera3d>)>,
    mut commands: Commands,
) {
    for camera in cameras.iter() {
        println!("Adding to camera");
        commands
            .get_entity(camera)
            .unwrap()
            .insert(Mask::crt())
            .insert(Wave {
                waves_x: 2.,
                waves_y: 0.5,
                speed_x: 2.,
                speed_y: 2.,
                amplitude_x: 50.,
                amplitude_y: 50.,
            })
            .insert(Pixelate { block_size: 20. });
    }
}

pub fn tick_camera_effects() {}
