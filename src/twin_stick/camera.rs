use bevy::prelude::{
    Entity, IntoSystemConfigs, Plugin, Query, Reflect, Res, Transform, Update, With,
};

use super::player::{player_exists, CursorTracker, MainCamera, Player};

#[derive(Clone, Copy, PartialEq, Eq, Reflect, Debug)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, camera_movement.run_if(player_exists));
    }
}

pub fn camera_movement(
    cursor: Query<Entity, With<CursorTracker>>,
    player: Query<Entity, With<Player>>,
    camera: Res<MainCamera>,
    mut transforms: Query<&mut Transform>,
) {
    let player_weight = 0.7;
    let delay = 0.15;
    let cursor_loc = transforms.get(cursor.single()).unwrap().translation;
    let player_loc = transforms.get(player.single()).unwrap().translation;
    let mut camera_loc = transforms.get_mut(camera.0).unwrap().translation;
    camera_loc = (cursor_loc * (1. - player_weight) + player_loc * player_weight) * delay
        + camera_loc * (1. - delay);
    transforms.get_mut(camera.0).unwrap().translation = camera_loc;
}
