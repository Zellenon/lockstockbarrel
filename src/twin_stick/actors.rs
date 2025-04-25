use avian2d::prelude::{
    Collider, CollisionLayers, ExternalForce, ExternalImpulse, LinearDamping, LinearVelocity,
    LockedAxes, Mass, RigidBody,
};
use bevy::{
    math::{Quat, Vec3, Vec3Swizzles},
    prelude::{
        in_state, App, Changed, Commands, Component, DespawnRecursiveExt, Entity, GlobalTransform,
        InheritedVisibility, IntoSystemConfigs, Parent, Query, Transform, Update, Vec2, Visibility,
        With, Without,
    },
    reflect::Reflect,
};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree, wrappers::name};
use bevy_stats::{Resource, Stat};

use super::{physics::GamePhysicsLayer as GPL, player::Player};
use crate::{
    game::stats::{Health, MoveSpeed},
    states::TimerState,
    transform2d::To2D,
    vision::{self, Identifying, Spotting},
};

#[derive(Clone, Copy, PartialEq, Reflect, Debug, Component)]
pub struct Actor {
    pub desired_direction: Vec2,
    pub desired_target: Option<Entity>,
}

#[derive(Clone, Copy, PartialEq, Eq, Reflect, Debug, Component)]
pub struct Faction(pub usize);

pub const UNIVERSAL_NEUTRAL_FACTION: usize = 0;
pub const PLAYER_FACTION: usize = 1;
pub const MISC_ENEMY_FACTION: usize = 2;
// Should universal neutral just be no faction component?

impl Default for Actor {
    fn default() -> Self {
        Self {
            desired_direction: Vec2::ZERO,
            desired_target: None,
        }
    }
}

pub fn basic_actor() -> ComponentTree {
    (
        Actor::default(),
        Visibility::Hidden,
        InheritedVisibility::default(),
        vision::Tracking::default(),
        Spotting::default(),
        Identifying::default(),
        Transform::default(),
        RigidBody::Dynamic,
        Mass(10.0),
        LinearDamping(3.),
        Collider::circle(15.),
        LockedAxes::ROTATION_LOCKED,
        Stat::<MoveSpeed>::new(50.),
        Resource::<Health>::new(5.),
        CollisionLayers::new(
            GPL::Enemy,
            [GPL::Enemy, GPL::Player, GPL::MapSolid, GPL::MapDynamic],
        ),
    )
        .store()
        + name("actor")
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

pub fn actor_movement(mut enemies: Query<(&mut ExternalForce, &Actor, &Stat<MoveSpeed>)>) {
    for (mut force, actor, speed) in enemies.iter_mut() {
        (force.x, force.y) = {
            let vec =
                Vec2::clamp_length_max(actor.desired_direction, 1.) * speed.current_value() * 600.;
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
