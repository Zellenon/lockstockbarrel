use crate::transform2d::To2D;
use avian2d::prelude::{
    Collider, ExternalForce, ExternalImpulse, LinearDamping, LinearVelocity, LockedAxes, Mass, RigidBody
};
use bevy::{
    math::{Quat, Vec3, Vec3Swizzles},
    prelude::{
        in_state, App, Bundle, Changed, Commands, Component, DespawnRecursiveExt, Entity,
        GlobalTransform, InheritedVisibility, IntoSystemConfigs, Parent, Query, Transform, Update,
        Vec2, Visibility, With, Without,
    },
    reflect::Reflect,
};
use bevy_stats::{Resource, Stat};

use crate::{
    game::stats::{Health, MoveSpeed},
    states::TimerState,
};

use super::player::Player;

#[derive(Clone, Copy, PartialEq, Reflect, Debug, Component)]
pub struct Actor {
    pub desired_direction: Vec2,
    pub desired_target: Option<Entity>,
}

#[derive(Clone, Copy, PartialEq, Eq, Reflect, Debug, Component)]
pub struct Faction(pub usize);

pub const PLAYER_FACTION: usize = 1;
pub const MISC_ENEMY_FACTION: usize = 2;
// Should universal neutral just be no faction component?
// pub const UNIVERSAL_NEUTRAL_FACTION: u16 = 0;

impl Default for Actor {
    fn default() -> Self {
        Self {
            desired_direction: Vec2::ZERO,
            desired_target: None,
        }
    }
}

#[derive(Clone, Debug, Bundle, Reflect)]
pub struct ActorBundle {
    pub actor: Actor,
    pub visibility: Visibility,
    pub computer_visibility: InheritedVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub rigidbody: RigidBody,
    pub mass_properties: Mass,
    pub damping: LinearDamping,
    pub velocity: LinearVelocity,
    pub external_force: ExternalForce,
    pub external_impulse: ExternalImpulse,
    pub collider: Collider,
    pub axes: LockedAxes,
}

impl Default for ActorBundle {
    fn default() -> Self {
        Self {
            actor: Default::default(),
            visibility: Visibility::Visible,
            transform: Default::default(),
            global_transform: Default::default(),
            rigidbody: RigidBody::Dynamic,
            mass_properties: Mass(300.0),
            damping: LinearDamping(13.),
            velocity: Default::default(),
            external_force: Default::default(),
            external_impulse: Default::default(),
            collider: Collider::circle(15.),
            axes: LockedAxes::ROTATION_LOCKED,
            computer_visibility: Default::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Reflect, Debug, Component)]
pub struct Tracking(pub Option<Entity>);

#[derive(Clone, Copy, PartialEq, Eq, Reflect, Debug, Component)]
pub struct Head;

#[derive(Clone, Copy, PartialEq, Reflect, Debug, Component)]
pub struct Legs {
    pub animation_stage: f32,
    pub stroke: isize,
    pub max_scale: f32,
}

impl Default for Legs {
    fn default() -> Self {
        Self {
            animation_stage: 0.,
            stroke: 1,
            max_scale: 1.,
        }
    }
}

pub(super) fn actor_plugin(app: &mut App) {
    app.register_type::<Actor>()
        .register_type::<Faction>()
        .register_type::<Legs>()
        .register_type::<Head>()
        .register_type::<Tracking>();

    app.add_systems(
        Update,
        (facing_update_system, animate_legs, health_death).run_if(in_state(TimerState::Playing)),
    );
}

pub fn facing_update_system(
    todo_entities: Query<Entity, (With<Tracking>, With<Transform>, With<Parent>)>,
    mut transforms: Query<(
        &GlobalTransform,
        Option<&Tracking>,
        Option<&LinearVelocity>,
        &mut Transform,
        Option<&Parent>,
    )>,
    parents: Query<&LinearVelocity>,
) {
    for entity in todo_entities.iter() {
        let entity = entity.clone();
        let target = transforms.get(entity).unwrap().1.unwrap().0;
        let direction: Vec2 = match target {
            Some(target_entity) => (transforms.get(target_entity).unwrap().0.translation()
                - transforms.get(entity).unwrap().0.translation())
            .xy(),
            None => match parents.get(transforms.get(entity).unwrap().4.unwrap().get()) {
                Ok(parent_vel) => parent_vel.0,
                Err(_) => Vec2::X,
            },
        };
        let transform = &mut transforms.get_mut(entity).unwrap().3;
        transform.rotation = Quat::from_2d(
            -2. * (direction.x
                / (direction.y + (direction.y.powi(2) + direction.x.powi(2)).sqrt()))
            .atan(),
        );
    }
}

fn animate_legs(
    mut legs: Query<(&mut Transform, &mut Legs, &Parent)>,
    parents: Query<&LinearVelocity>,
) {
    for (mut transform, mut legs, parent) in legs.iter_mut() {
        if legs.animation_stage > 199. {
            legs.stroke = -1;
        }
        if legs.animation_stage < -199. {
            legs.stroke = 1;
        }
        match parents.get(parent.get()) {
            Ok(parent_vel) => {
                let parent_speed = parent_vel.0.length();
                if parent_speed > 0.02 {
                    legs.animation_stage += parent_speed * legs.stroke as f32 / 20.;
                } else {
                    legs.animation_stage = legs.animation_stage as f32 * 0.9;
                }
                transform.scale = Vec3::new(1., legs.animation_stage as f32 / 100., 1.);
            }
            Err(e) => println!("{}", e),
        };
    }
}

pub fn actor_movement(mut enemies: Query<(&mut ExternalImpulse, &Actor, &Stat<MoveSpeed>)>) {
    for (mut force, actor, speed) in enemies.iter_mut() {
        (force.x, force.y) = {
            let vec =
                Vec2::clamp_length_max(actor.desired_direction, 1.) * speed.current_value() * 750.;
            (vec.x, vec.y)
        };
    }
}

pub fn health_death(
    mut commands: Commands,
    health_query: Query<(Entity, &Resource<Health>), (Without<Player>, Changed<Resource<Health>>)>,
) {
    for (entity, health) in health_query.iter() {
        if health.current_value() <= 0. {
            commands.entity(entity).despawn_recursive();
        }
    }
}
