use avian2d::prelude::CollisionLayers;
use bevy::{
    ecs::{
        bundle::Bundle,
        children,
        name::Name,
        system::{Commands, Res},
    },
    transform::components::Transform,
};
use bevy_stats::Stat;

use super::{
    actor_bits::{basic_head, basic_legs},
    util::tracking,
    weapons::{peashooter, sonar_launcher},
};
use crate::{
    action_system::triggers::key_action::PlayerActionTrigger,
    assets::images::ImageResources,
    game::stats::{IdentifyPower, MoveSpeed, SpotTime},
    twin_stick::{
        actors::{basic_actor, Faction, Tracking, PLAYER_FACTION},
        ai::keyboard::{player_input_bundle, KeyboardAI, PlayerAction},
        physics::GamePhysicsLayer as GPL,
        player::{Cursor, Player},
    },
    util::gimmie::image,
    vision::eyes::{Eye, EyeDistance, EyeFOV},
};

pub fn spawn_player(mut commands: Commands, cursor: Res<Cursor>) {
    commands.spawn((player(&cursor),));
}

fn player_base() -> impl Bundle {
    (
        Player,
        Name::new("Player"),
        basic_head(),
        basic_actor(),
        Stat::<MoveSpeed>::new(80.),
        Stat::<EyeFOV>::new(2.),
        Stat::<EyeDistance>::new(200.),
        Stat::<IdentifyPower>::new(33.),
        Stat::<SpotTime>::new(3.),
        Eye::default(),
        image(ImageResources::player_head),
        player_input_bundle(),
        CollisionLayers::new(
            GPL::Player,
            [
                GPL::Player,
                GPL::Enemy,
                GPL::MapSolid,
                GPL::MapDynamic,
                GPL::Bullet,
            ],
        ),
        KeyboardAI,
        Faction(PLAYER_FACTION),
    )
}

pub fn player(cursor: &Res<Cursor>) -> impl Bundle {
    (
        player_base(),
        tracking(cursor.0),
        children![
            (basic_legs(), image(ImageResources::player_legs)),
            (
                player_limb(cursor, PlayerAction::Shoot1),
                children![peashooter(&cursor)],
                Name::new("Main Hand")
            ),
            (
                player_limb(cursor, PlayerAction::Shoot3),
                children![sonar_launcher(&cursor)],
                Name::new("Speaker")
            )
        ],
    )
}

pub fn player_limb(cursor: &Res<Cursor>, trigger: PlayerAction) -> impl Bundle {
    (
        PlayerActionTrigger::new([trigger]),
        Tracking(Some(cursor.0)),
        Transform::default(),
    )
}
