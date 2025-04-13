use bevy::ecs::system::{Commands, Resource};
use bevy_composable::app_impl::ComplexSpawnable;

use super::arena_objects::wall;

type ArenaMap = Vec<Vec<bool>>;

#[derive(Resource)]
pub struct Arena {
    pub arena_map: ArenaMap,
    pub resolution: f32,
}

pub fn to_map(map: Vec<Vec<u8>>) -> ArenaMap {
    map.iter()
        .map(|w| w.iter().map(|x| *x == 1).collect())
        .collect()
}

pub fn spawn_arena_from_map(commands: &mut Commands, level: &Arena) {
    let y_len = (level.arena_map.len() as f32) * level.resolution;
    let x_len = (level
        .arena_map
        .iter()
        .map(|w| w.len())
        .max()
        .unwrap() as f32)
        * level.resolution;
    let mut i = 0.;
    for row in level.arena_map.iter() {
        let mut j = 0.;
        for block in row.iter() {
            if *block {
                commands.compose(wall(
                    j * level.resolution - (x_len * 0.5),
                    (y_len * 0.5) - i * level.resolution,
                    level.resolution,
                    level.resolution,
                ));
            };
            j = j + 1.;
        }
        i = i + 1.;
    }
}
