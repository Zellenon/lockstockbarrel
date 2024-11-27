use bevy::{
    app::{App, Plugin, Startup},
    ecs::system::Commands,
    prelude::{not, IntoSystemConfigs, Update},
};
use bevy_composable::app_impl::ComplexSpawnable;
use stats::stats_plugin;

use crate::{
    action_system::{
        actions::telegraphed,
        prefabs::{spawn_delay, spawn_prox},
    },
    arena::{spawn_arena_from_map, to_map, Arena},
    content::{enemies::stumbler, player::spawn_player},
    transform2d::pos,
    twin_stick::{actors::PLAYER_FACTION, player::player_exists},
};

pub mod stats;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        stats_plugin(app);

        // app.add_systems(OnEnter(AppState::Game), (player_setup, test_load_level));
        app.add_systems(Startup, test_load_level);

        app.add_systems(Update, spawn_player.run_if(not(player_exists)));
    }
}

fn test_load_level(mut commands: Commands) {
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
    spawn_arena_from_map(&mut commands, &level);

    commands.compose(
        pos(450., 450.)
            + spawn_prox(
                1 << PLAYER_FACTION,
                200.,
                spawn_delay(1.0, stumbler()) + telegraphed(),
            )
            + telegraphed(),
    );
}
