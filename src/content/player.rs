use crate::{
    assets::images::ImageResources,
    game::stats::{IdentifyPower, MoveSpeed, SpotTime},
    twin_stick::{
        actors::{basic_actor, Faction, PLAYER_FACTION},
        ai::keyboard::{create_player_action_input_manager_bundle, KeyboardAI},
        physics::GamePhysicsLayer as GPL,
        player::{Cursor, Player},
    },
    util::image,
};
use avian2d::prelude::CollisionLayers;
use bevy::ecs::system::{Commands, Res};
use bevy_composable::{
    app_impl::{ComplexSpawnable, ComponentTreeable},
    tree::ComponentTree,
    wrappers::name,
};
use bevy_stats::Stat;

use super::{
    actor_bits::{basic_head, basic_legs},
    util::tracking,
    weapons::{peashooter, sonar_launcher},
};

pub fn spawn_player(mut commands: Commands, cursor: Res<Cursor>) {
    let player_id =
        commands.compose((player_tree(&cursor) << peashooter(&cursor)) << sonar_launcher(&cursor));
    commands
        .get_entity(player_id)
        .unwrap()
        .insert(create_player_action_input_manager_bundle());
}

fn player_tree_base() -> ComponentTree {
    (Player, KeyboardAI).store()
        + Stat::<MoveSpeed>::new(80.).store()
        + Stat::<SpotTime>::new(3.).store()
        + Stat::<IdentifyPower>::new(33.).store()
        + Faction(PLAYER_FACTION).store()
        + CollisionLayers::new(
            GPL::Player,
            [
                GPL::Player,
                GPL::Enemy,
                GPL::MapSolid,
                GPL::MapDynamic,
                GPL::Bullet,
            ],
        )
        .store()
        + name("Player")
        + basic_actor()
}

pub fn player_tree(cursor: &Res<Cursor>) -> ComponentTree {
    player_tree_base()
        << (basic_head() + tracking(cursor.0) + image(ImageResources::player_head))
        << (basic_legs() + image(ImageResources::player_legs))
    // << wallgun()
}
