use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_composable::{
    app_impl::ComplexSpawnable,
    tree::{ComponentTree, EntityCommandSet},
};
use bevy_stats::Stat;
use std::sync::Arc;
use twin_stick::{
    actors::ActorBundle,
    ai::keyboard::KeyboardAI,
    player::{Cursor, Player},
};

use crate::{
    content::{
        actor_bits::{basic_head, basic_legs},
        shift_tracking,
        stats::Speed,
        weapons::peashooter,
        ContentPlugin,
    },
    states::AppState,
};

use self::{
    level::{spawn_arena_from_map, to_map, Level},
    level_event::LeveleventPlugin,
    level_event_manager::{test_lemanager_setup, LeveleventManagerPlugin},
};

pub mod level;
pub mod level_event;
pub mod level_event_manager;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ContentPlugin)
            .add_plugin(LeveleventPlugin)
            .add_plugin(LeveleventManagerPlugin);

        app.add_system(player_setup.in_schedule(OnEnter(AppState::Game)))
            .add_system(test_lemanager_setup.in_schedule(OnEnter(AppState::Game)))
            .add_system(test_load_level.in_schedule(OnEnter(AppState::Game)));
    }
}

fn player_setup(mut commands: Commands, asset_server: Res<AssetServer>, cursor: Res<Cursor>) {
    commands.spawn_complex(player_tree(
        asset_server.load("img/player_head.png").clone(),
        asset_server.load("img/player_legs.png").clone(),
        cursor,
    ));
}

fn player_tree_base() -> ComponentTree {
    let func = move |parent: &mut EntityCommands| {
        parent.insert((
            Player,
            Name::new("Player"),
            ActorBundle::default(),
            Stat::<Speed>::new(1500.),
            KeyboardAI,
        ));
    };
    (Arc::new(func) as EntityCommandSet).into()
}

pub fn player_tree(
    head_tex: Handle<Image>,
    leg_tex: Handle<Image>,
    cursor: Res<Cursor>,
) -> ComponentTree {
    player_tree_base()
        << (basic_head(head_tex) + shift_tracking(Some(cursor.0)))
        << basic_legs(leg_tex)
        << peashooter()
    // << wallgun()
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
    let level = Level {
        arena_map: to_map(demo_map),
        resolution: 600.,
    };
    spawn_arena_from_map(commands, &level);
}
