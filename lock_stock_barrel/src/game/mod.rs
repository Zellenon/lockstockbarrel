use bevy::prelude::*;
use bevy_composable::*;
use bevy_stats::{Speed, Stat};
use iyes_loopless::prelude::AppLooplessStateExt;
use twin_stick::{
    actors::{ActorBundle, Legs, Tracking},
    ai::KeyboardAI,
    player::{Cursor, Player},
    weapons::make_peashooter,
};

use crate::{
    content::{enemies::basic_walker, shift_pos},
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
        app.add_plugin(LeveleventPlugin)
            .add_plugin(LeveleventManagerPlugin);

        app.add_enter_system(AppState::Game, player_setup)
            .add_enter_system(AppState::Game, test_lemanager_setup)
            .add_enter_system(AppState::Game, test_load_level);
    }
}

fn player_setup(mut commands: Commands, asset_server: Res<AssetServer>, cursor: Res<Cursor>) {
    commands
        .spawn((
            Player,
            ActorBundle::default(),
            Stat::<Speed>::new(1500.),
            KeyboardAI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Tracking(Some(cursor.0)),
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Vec2::new(40., 40.).into(),
                        ..Default::default()
                    },
                    texture: asset_server.load("img/player_head.png"),
                    ..Default::default()
                },
            ));
            parent.spawn((
                Legs::default(),
                Tracking(None),
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Vec2::new(30., 35.).into(),
                        ..Default::default()
                    },
                    texture: asset_server.load("img/player_legs.png"),
                    ..Default::default()
                },
            ));
            parent.spawn(make_peashooter());
        });
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
