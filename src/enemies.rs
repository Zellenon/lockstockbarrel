use bevy::prelude::*;
use bevy_rapier2d::prelude::{
    ActiveCollisionTypes, ActiveEvents, Collider, ColliderMassProperties,
    ContactForceEventThreshold, Damping, ExternalForce, LockedAxes, RigidBody, Velocity,
};

use crate::{
    actors::{Actor, Legs, Tracking},
    ai::TrackerAI,
    stats::{Speed, Stat},
};

#[derive(Component)]
pub struct Enemy {
    pub desired_direction: Vec2,
    pub desired_target: Option<Entity>,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            desired_direction: Vec2::ZERO,
            desired_target: None,
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(enemy_setup)
            .add_system(enemy_movement);
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
            Enemy::default(),
            Actor,
            TrackerAI,
            Speed::new(500.),
            SpatialBundle {
                visibility: Visibility { is_visible: true },
                transform: Transform::from_translation(location.extend(0.)),
                ..Default::default()
            },
            RigidBody::Dynamic,
            ColliderMassProperties::Density(0.3),
            Velocity::default(),
            Damping {
                linear_damping: 20.,
                angular_damping: 1.0,
            },
            ExternalForce::default(),
            Collider::ball(15.),
            LockedAxes::ROTATION_LOCKED,
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

fn enemy_movement(mut enemies: Query<(&mut ExternalForce, &Enemy, &Speed)>) {
    for (
        mut force,
        enemy,
        Speed {
            current: speed,
            max: _,
        },
    ) in enemies.iter_mut()
    {
        force.force = Vec2::normalize_or_zero(enemy.desired_direction) * *speed;
    }
}
