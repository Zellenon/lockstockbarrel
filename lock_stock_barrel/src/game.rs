use bevy::prelude::*;
use twin_stick::{enemies::spawn_enemy, obstacle_builder};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_walls);
        app.add_startup_system(enemy_setup);
    }
}

fn spawn_walls(mut commands: Commands) {
    obstacle_builder(&mut commands, -1000., 0., 50., 2000.);
    obstacle_builder(&mut commands, 1000., 0., 50., 2000.);
    obstacle_builder(&mut commands, 0., 1000., 2000., 50.);
    obstacle_builder(&mut commands, 0., -1000., 2000., 50.);
}

fn enemy_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_enemy(&mut commands, Vec2::new(500., 0.), &asset_server);
    spawn_enemy(&mut commands, Vec2::new(0., 500.), &asset_server);
    spawn_enemy(&mut commands, Vec2::new(-500., 0.), &asset_server);
}
