use crate::{
    assets::images::ImageResources,
    game::stats::MoveSpeed,
    twin_stick::{
        actors::ActorBundle,
        ai::keyboard::KeyboardAI,
        player::{Cursor, Player},
    },
    util::image,
};
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
    // weapons::peashooter,
};

pub fn spawn_player(mut commands: Commands, cursor: Res<Cursor>) {
    commands.compose(player_tree(cursor));
}

fn player_tree_base() -> ComponentTree {
    (Player, ActorBundle::default(), KeyboardAI).store()
        + Stat::<MoveSpeed>::new(50.).store()
        + name("Player")
}

pub fn player_tree(cursor: Res<Cursor>) -> ComponentTree {
    player_tree_base()
        << (basic_head() + tracking(cursor.0) + image(ImageResources::player_head))
        << (basic_legs() + image(ImageResources::player_legs))
    // << peashooter()
    // << wallgun()
}
