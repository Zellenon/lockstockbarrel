use bevy::prelude::*;
use bevy_rapier2d::prelude::{
    Collider, ColliderMassProperties, Damping, ExternalForce, ExternalImpulse, LockedAxes,
    RigidBody, Velocity,
};

use crate::{
    actors::{Actor, ActorBundle, Legs, Tracking},
    ai::TrackerAI,
    stats::{Health, Speed, Stat},
};

#[derive(Component)]
pub struct Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(enemy_setup);
    }
}

fn enemy_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_enemy(&mut commands, Vec2::new(500., 0.), &asset_server);
    spawn_enemy(&mut commands, Vec2::new(0., 500.), &asset_server);
    spawn_enemy(&mut commands, Vec2::new(-500., 0.), &asset_server);
}

fn spawn_enemy(commands: &mut Commands, location: Vec2, asset_server: &Res<AssetServer>) {
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
