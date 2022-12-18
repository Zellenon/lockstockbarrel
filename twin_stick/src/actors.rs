use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

use crate::{player::Player, utils::get_angle};
use bevy_stats::{Health, Speed, Stat};

#[derive(Component)]
pub struct Actor {
    pub desired_direction: Vec2,
    pub desired_target: Option<Entity>,
}

impl Default for Actor {
    fn default() -> Self {
        Self {
            desired_direction: Vec2::ZERO,
            desired_target: None,
        }
    }
}

#[derive(Bundle)]
pub struct ActorBundle {
    pub actor: Actor,
    pub visibility: Visibility,
    pub computer_visibility: ComputedVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub rigidbody: RigidBody,
    pub mass_properties: ColliderMassProperties,
    pub velocity: Velocity,
    pub damping: Damping,
    pub external_force: ExternalForce,
    pub external_impulse: ExternalImpulse,
    pub collider: Collider,
    pub axes: LockedAxes,
}

impl Default for ActorBundle {
    fn default() -> Self {
        Self {
            actor: Default::default(),
            visibility: Visibility { is_visible: true },
            transform: Default::default(),
            global_transform: Default::default(),
            rigidbody: RigidBody::Dynamic,
            mass_properties: ColliderMassProperties::Density(0.3),
            velocity: Default::default(),
            damping: Damping {
                linear_damping: 20.,
                angular_damping: 1.0,
            },
            external_force: Default::default(),
            external_impulse: Default::default(),
            collider: Collider::ball(15.),
            axes: LockedAxes::ROTATION_LOCKED,
            computer_visibility: Default::default(),
        }
    }
}

#[derive(Component, Inspectable)]
pub struct Tracking(pub Option<Entity>);

#[derive(Component, Reflect)]
pub struct Head;

#[derive(Component)]
pub struct Legs {
    pub animation_stage: isize,
    pub stroke: isize,
    pub max_scale: f32,
}

impl Default for Legs {
    fn default() -> Self {
        Self {
            animation_stage: 0,
            stroke: 1,
            max_scale: 1.,
        }
    }
}

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(facing_update_system)
            .add_system(actor_movement)
            .add_system(animate_legs)
            .add_system(health_death);
    }
}

fn facing_update_system(
    todo_entities: Query<Entity, (With<Tracking>, With<Transform>, With<Parent>)>,
    mut transforms: Query<(
        &GlobalTransform,
        Option<&Tracking>,
        Option<&Velocity>,
        &mut Transform,
        Option<&Parent>,
    )>,
    parents: Query<&Velocity>,
) {
    for entity in todo_entities.iter() {
        let entity = entity.clone();
        let target = transforms.get(entity).unwrap().1.unwrap().0;
        let direction = match target {
            Some(target_entity) => {
                transforms.get(target_entity).unwrap().0.translation()
                    - transforms.get(entity).unwrap().0.translation()
            }
            None => match parents.get(transforms.get(entity).unwrap().4.unwrap().get()) {
                Ok(parent_vel) => (parent_vel.linvel * 1.).extend(0.),
                Err(_) => Vec3::Y,
            },
        };
        let transform: &mut Transform = &mut transforms.get_mut(entity).unwrap().3;
        let result = get_angle(direction);
        if !result.is_nan() {
            transform.rotation = result;
        }
    }
}

fn animate_legs(mut legs: Query<(&mut Transform, &mut Legs, &Parent)>, parents: Query<&Velocity>) {
    for (mut transform, mut legs, parent) in legs.iter_mut() {
        if legs.animation_stage > 199 {
            legs.stroke = -1;
        }
        if legs.animation_stage < -199 {
            legs.stroke = 1;
        }
        match parents.get(parent.get()) {
            Ok(parent_vel) => {
                let parent_speed = parent_vel.linvel.length() as isize;
                if parent_speed > 0 {
                    legs.animation_stage += parent_speed * legs.stroke / 30;
                } else {
                    legs.animation_stage = (legs.animation_stage as f32 * 0.9) as isize;
                }
                transform.scale = Vec3::new(1., legs.animation_stage as f32 / 100., 1.);
            }
            Err(e) => println!("{}", e),
        };
    }
}

fn actor_movement(mut enemies: Query<(&mut ExternalForce, &Actor, &Stat<Speed>)>) {
    for (mut force, actor, speed) in enemies.iter_mut() {
        force.force = Vec2::normalize_or_zero(actor.desired_direction) * speed.current_value();
    }
}

fn health_death(
    mut commands: Commands,
    health_query: Query<(Entity, &Stat<Health>), (Without<Player>, Changed<Stat<Health>>)>,
) {
    for (entity, health) in health_query.iter() {
        if health.current_value() < 0. {
            commands.entity(entity).despawn_recursive();
        }
    }
}
