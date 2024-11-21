use bevy::prelude::{not, Update};
use bevy::{
    app::{App, Plugin, Startup},
    ecs::system::Commands,
    prelude::IntoSystemConfigs,
};
use stats::stats_plugin;

use crate::{content::player::spawn_player, twin_stick::player::player_exists};

use self::arena::{spawn_arena_from_map, to_map, Arena};

pub mod arena;
pub mod stats;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugins((ContentPlugin, ArenaeventPlugin, ArenaeventManagerPlugin));

        stats_plugin(app);

        // app.add_systems(OnEnter(AppState::Game), (player_setup, test_load_level));
        app.add_systems(Startup, test_load_level);

        app.add_systems(Update, spawn_player.run_if(not(player_exists)));
    }
}

fn test_load_level(commands: Commands) {
    let demo_map: Vec<Vec<u8>> = vec![
        vec![1, 1, 1, 1, 1],
        vec![1, 0, 0, 0, 1],
        vec![1, 0, 1, 0, 1],
        vec![1, 0, 0, 0, 1],
        vec![1, 1, 0, 0, 1],
        vec![0, 1, 1, 1, 1],
    ];
    let level = Arena {
        arena_map: to_map(demo_map),
        resolution: 600.,
    };
    spawn_arena_from_map(commands, &level);
}
