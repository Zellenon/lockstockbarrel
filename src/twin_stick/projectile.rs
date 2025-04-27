use avian2d::prelude::{Collider, CollisionStarted, LinearVelocity, Mass, RigidBody, SweptCcd};
use bevy::{
    color::{palettes::css::RED, Color},
    ecs::{schedule::SystemSet, system::ResMut},
    hierarchy::{HierarchyQueryExt, Parent},
    math::{Vec2Swizzles, Vec3Swizzles},
    prelude::{
        in_state, App, Commands, Component, DespawnRecursiveExt, Entity, Event, EventReader,
        EventWriter, IntoSystemConfigs, Query, Reflect, Res, Transform, Update, Vec2, Visibility,
    },
    sprite::Sprite,
    time::{Time, Timer, TimerMode},
    utils::default,
};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree, wrappers::name};
use std::time::Duration;

use super::{actors::Actor, events::AttackEvent, weapons::Weapon};
use crate::{action_system::actions::spawn::SpawnedBy, states::TimerState};

#[derive(Debug, SystemSet, Reflect, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ProjectileSystems;

#[derive(Component, Clone, PartialEq, Eq, Reflect, Debug)]
pub struct Lifespan(Timer);

#[derive(Component, Clone, Copy, PartialEq, Eq, Reflect, Debug)]
pub struct Projectile {
    pub on_prop: ProjectileImpactBehavior,
    pub on_actor: ProjectileImpactBehavior,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Reflect, Debug)]
pub enum ProjectileImpactBehavior {
    Die,
    Bounce,
}

#[derive(Event, Clone, Copy, PartialEq, Eq, Reflect, Debug)]
pub struct ProjectileImpactEvent {
    pub projectile: Entity,
    pub impacted: Entity,
}

#[derive(Clone, Copy, PartialEq, Eq, Reflect, Debug, Event)]
pub struct ProjectileClashEvent(pub Entity, pub Entity);

#[derive(Clone, PartialEq, Eq, Reflect, Debug, Event)]
pub struct ContactDamage(pub Option<Timer>);

pub fn projectile_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            tick_lifetimes,
            (
                projectile_collision_event_dispatcher,
                (
                    kill_projectiles_post_impact,
                    projectile_hits_trigger_attacks,
                ),
            )
                .chain(),
        )
            .run_if(in_state(TimerState::Playing))
            .in_set(ProjectileSystems),
    );

    app.add_event::<ProjectileImpactEvent>()
        .add_event::<ProjectileClashEvent>();
}

pub fn projectile(lifespan: f32, projectile: Projectile) -> ComponentTree {
    (
        projectile,
        Visibility::Visible,
        RigidBody::Dynamic,
        Lifespan::new(lifespan),
        Collider::circle(3.),
        SweptCcd::default(),
        Mass(0.1),
        Sprite {
            color: Color::Srgba(RED),
            custom_size: Some(Vec2::new(6., 6.)),
            ..default()
        },
    )
        .store()
        + name("Projectile")
}

impl Lifespan {
    pub fn new(secs: f32) -> Self {
        Self(Timer::new(Duration::from_secs_f32(secs), TimerMode::Once))
    }
}

impl Default for Lifespan {
    fn default() -> Self {
        Self(Timer::new(Duration::from_secs_f32(0.8), TimerMode::Once))
    }
}

impl Default for Projectile {
    fn default() -> Self {
        Self {
            on_prop: ProjectileImpactBehavior::Die,
            on_actor: ProjectileImpactBehavior::Die,
        }
    }
}

fn tick_lifetimes(
    mut commands: Commands,
    time: Res<Time>,
    mut lifespans: Query<(&mut Lifespan, Entity)>,
) {
    for (mut lifespan, entity) in lifespans.iter_mut() {
        lifespan.0.tick(time.delta());

        if lifespan.0.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn projectile_collision_event_dispatcher(
    mut collision_events: EventReader<CollisionStarted>,
    projectile_query: Query<&Projectile>,
    mut projectile_events: EventWriter<ProjectileImpactEvent>,
    mut clash_events: EventWriter<ProjectileClashEvent>,
) {
    for collision_event in collision_events.read() {
        let CollisionStarted(e1, e2) = collision_event;
        match (projectile_query.get(*e1), projectile_query.get(*e2)) {
            (Ok(_), Ok(_)) => {
                clash_events.send(ProjectileClashEvent(*e1, *e2));
            }
            (Ok(_), _) => {
                projectile_events.send(ProjectileImpactEvent {
                    projectile: *e1,
                    impacted: *e2,
                });
            }
            (Err(_), Ok(_)) => {
                projectile_events.send(ProjectileImpactEvent {
                    impacted: *e1,
                    projectile: *e2,
                });
            }
            (Err(_), Err(_)) => continue,
        };
    }
}

//TODO: Attack directions are still incorrect
fn projectile_hits_trigger_attacks(
    mut projectile_events: EventReader<ProjectileImpactEvent>,
    mut attack_events: EventWriter<AttackEvent>,
    transforms: Query<&Transform>,
    bullets: Query<(&SpawnedBy, Option<&LinearVelocity>)>,
    parents: Query<&Parent>,
    weapons: Query<&Weapon>,
    actors: Query<&Actor>,
) {
    for ProjectileImpactEvent {
        projectile,
        impacted,
    } in projectile_events.read()
    {
        let (target_pos, projectile_pos) = (
            transforms.get(*projectile).unwrap(),
            transforms.get(*impacted).unwrap(),
        );
        let location = projectile_pos.translation.xy();
        if let Ok((SpawnedBy(spawner), velocity)) = bullets.get(*projectile) {
            let weapon = std::iter::once(*spawner)
                .chain(parents.iter_ancestors(*spawner))
                .filter(|w| weapons.get(*w).is_ok())
                .next();
            let attacker = std::iter::once(*spawner)
                .chain(parents.iter_ancestors(*spawner))
                .filter(|w| actors.get(*w).is_ok())
                .next();
            let direction = match velocity {
                Some(vel) => vel.yx(),
                None => (target_pos.translation.xy() - location).normalize(),
            };
            if let (Some(attacker), Some(weapon)) = (attacker, weapon) {
                attack_events.send(AttackEvent {
                    attacker,
                    weapon,
                    defender: *impacted,
                    location,
                    direction,
                });
            } else {
                println!("ATTACK EVENT NOT SENT. {:?}, {:?}", attacker, weapon);
            }
        }
    }
}

fn kill_projectiles_post_impact(
    mut events: EventReader<ProjectileImpactEvent>,
    mut commands: Commands,
    query: Query<&Projectile>,
) {
    for ProjectileImpactEvent {
        projectile: projectile_id,
        impacted: _,
    } in events.read()
    {
        let proj = query.get(*projectile_id);
        match proj {
            Ok(projectile) => {
                if projectile.on_actor == ProjectileImpactBehavior::Die {
                    commands.entity(*projectile_id).despawn_recursive();
                }
            }
            Err(_) => (),
        }
    }
}
