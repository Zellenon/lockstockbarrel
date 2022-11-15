use bevy::prelude::*;
use bevy_rapier2d::prelude::{
    Collider, ColliderMassProperties, Damping, ExternalForce, LockedAxes, RigidBody, Velocity,
};

use crate::{
    actors::{Actor, Legs, Tracking},
    stats::Speed,
    AI::TrackerAI,
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

fn enemy_setup(mut commands: Commands) {
    spawn_enemy(&mut commands, Vec2::new(500., 0.));
    spawn_enemy(&mut commands, Vec2::new(0., 500.));
    spawn_enemy(&mut commands, Vec2::new(-500., 0.));
}

fn spawn_enemy(commands: &mut Commands, location: Vec2) {
    commands
        .spawn() // Player
        .insert(Actor)
        .insert(TrackerAI)
        .insert(Speed(500.))
        .insert_bundle(SpatialBundle {
            visibility: Visibility { is_visible: true },
            transform: Transform::from_translation(location.extend(0.)),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(ColliderMassProperties::Density(0.3))
        .insert(Velocity::default())
        .insert(Damping {
            linear_damping: 20.,
            angular_damping: 1.0,
        })
        .insert(ExternalForce::default())
        .insert(Enemy::default())
        .insert(Collider::ball(15.))
        .insert(LockedAxes::ROTATION_LOCKED)
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Vec2::new(40., 40.).into(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert_bundle(SpatialBundle {
                    visibility: Visibility { is_visible: true },
                    ..Default::default()
                })
                .insert(Tracking(None));

            parent
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Vec2::new(30., 35.).into(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Tracking(None))
                .insert_bundle(SpatialBundle {
                    visibility: Visibility { is_visible: true },
                    transform: Transform::from_xyz(0., 0., -1.),
                    ..Default::default()
                })
                .insert(Legs::default());
        });
}

fn enemy_movement(mut enemies: Query<(&mut ExternalForce, &Enemy, &Speed)>) {
    for (mut force, enemy, Speed(speed)) in enemies.iter_mut() {
        force.force = Vec2::normalize_or_zero(enemy.desired_direction) * *speed;
    }
}
