use avian2d::prelude::{CollisionLayers, RigidBody};
use bevy::ecs::{
    bundle::Bundle,
    children,
    name::Name,
    system::{Commands, Res},
};
use bevy_stats::Stat;

use super::{
    actor_bits::{basic_head, basic_legs},
    util::tracking,
    weapons::{peashooter, sonar_launcher},
};
use crate::{
    assets::images::ImageResources,
    game::stats::{IdentifyPower, MoveSpeed, SpotTime},
    twin_stick::{
        actors::{basic_actor, Faction, PLAYER_FACTION},
        ai::keyboard::{create_player_action_input_manager_bundle, KeyboardAI},
        physics::GamePhysicsLayer as GPL,
        player::{Cursor, Player},
    },
    util::{gimmie::image, spawning::Spawnable},
    vision::eyes::{Eye, EyeDistance, EyeFOV},
};

pub fn spawn_player(mut commands: Commands, cursor: Res<Cursor>) {
    commands
        .spawn((
            player_tree(&cursor),
            children![peashooter(&cursor), sonar_launcher(&cursor)],
        ))
        .insert(create_player_action_input_manager_bundle());
}

fn player_tree_base(cursor: &Res<Cursor>) -> impl Spawnable {
    (
        basic_actor(),
        Player,
        KeyboardAI,
        Stat::<MoveSpeed>::new(80.),
        Stat::<SpotTime>::new(3.),
        Stat::<IdentifyPower>::new(33.),
        Faction(PLAYER_FACTION),
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
        Name::new("Player"),
    )
}

pub fn player_tree(cursor: &Res<Cursor>) -> impl Spawnable {
    (
        player_tree_base(cursor),
        basic_head(),
        tracking(cursor.0),
        image(ImageResources::player_head),
        (
            //RigidBody::Kinematic,
            Stat::<EyeDistance>::new(200.),
            Stat::<EyeFOV>::new(2.),
            Eye::default(),
        ),
        children![
        (basic_legs() , image(ImageResources::player_legs))
    // << wallgun()
            ],
    )
}
