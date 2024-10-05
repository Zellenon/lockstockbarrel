use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        entity::Entity,
        query::Without,
        system::{Commands, Query},
    },
    prelude::OnEnter,
    transform::components::Transform,
};
use bevy_twin_stick::bevy_mod_transform2d::transform2d::Transform2d;

use crate::{
    content::{player::player_setup, ContentPlugin},
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
        app.add_plugins((ContentPlugin, LeveleventPlugin, LeveleventManagerPlugin));

        app.add_systems(
            OnEnter(AppState::Game),
            (player_setup, test_lemanager_setup, test_load_level),
        );

        app.add_systems(Update, ensure_transform2d);
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
    let level = Level {
        arena_map: to_map(demo_map),
        resolution: 600.,
    };
    spawn_arena_from_map(commands, &level);
}

pub fn ensure_transform2d(
    mut commands: Commands,
    query: Query<(Entity, &Transform2d), Without<Transform>>,
) {
    for (e, transform) in query.iter() {
        commands.entity(e).insert({
            let t: Transform = (*transform).into();
            t
        });
    }
}
