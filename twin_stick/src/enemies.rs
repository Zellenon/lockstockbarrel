use bevy::prelude::*;

use crate::{
    actors::{ActorBundle, Legs, Tracking},
    ai::TrackerAI,
};
use bevy_stats::{Health, Speed, Stat};

#[derive(Component)]
pub struct Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {}
}

pub fn spawn_enemy(commands: &mut Commands, location: Vec2, asset_server: &Res<AssetServer>) {
    commands
        .spawn((
            Enemy,
            ActorBundle {
                transform: Transform::from_translation(location.extend(0.)),
                ..Default::default()
            },
            TrackerAI,
            Stat::<Speed>::new(500.),
            Stat::<Health>::new(50.),
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Vec2::new(40., 40.).into(),
                        ..Default::default()
                    },
                    texture: asset_server.load("img/placeholder_head.png"),
                    ..Default::default()
                },
                Tracking(None),
            ));

            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Vec2::new(30., 35.).into(),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0., 0., -1.),
                    texture: asset_server.load("img/placeholder_legs.png"),
                    ..Default::default()
                },
                Tracking(None),
                Legs::default(),
            ));
        });
}
