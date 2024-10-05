use bevy::{
    asset::{AssetServer, Handle},
    core::Name,
    ecs::system::{Commands, Res},
    render::texture::Image,
};
use bevy_composable::{
    app_impl::{ComplexSpawnable, ComponentTreeable},
    tree::ComponentTree,
};
use bevy_stats::Stat;
use bevy_twin_stick::{
    actors::ActorBundle,
    ai::keyboard::KeyboardAI,
    player::{Cursor, Player},
};

use super::{
    actor_bits::{basic_head, basic_legs},
    shift_tracking,
    stats::Speed,
    weapons::peashooter,
};

pub fn player_setup(mut commands: Commands, asset_server: Res<AssetServer>, cursor: Res<Cursor>) {
    commands.compose(player_tree(
        asset_server.load("img/player_head.png").clone(),
        asset_server.load("img/player_legs.png").clone(),
        cursor,
    ));
}

fn player_tree_base() -> ComponentTree {
    (
        Player,
        Name::new("Player"),
        ActorBundle::default(),
        Stat::<Speed>::new(1500.),
        KeyboardAI,
    )
        .store()
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
