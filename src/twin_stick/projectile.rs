use avian2d::prelude::{Collider, CollisionStarted, ExternalImpulse, LinearVelocity, Mass, RigidBody, SweptCcd};
use bevy::{
    color::{palettes::css::RED, Color}, math::Vec3Swizzles, prelude::{
        in_state, App, Bundle, Commands, Component, DespawnRecursiveExt, Entity, Event,
        EventReader, EventWriter, GlobalTransform, InheritedVisibility, IntoSystemConfigs, Plugin,
        Query, Reflect, Res, Transform, Update, Vec2, Visibility,
    }, sprite::Sprite, time::{Time, Timer, TimerMode}, utils::default
};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree, wrappers::name};
use std::{marker::PhantomData, time::Duration};

use crate::states::TimerState;

#[derive(Component, Clone, PartialEq, Eq, Reflect, Debug)]
pub struct Lifespan(Timer);

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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Reflect, Debug)]
pub enum ProjectileImpactBehavior {
    Die,
    Bounce,
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Reflect, Debug)]
pub struct Projectile {
    pub on_prop: ProjectileImpactBehavior,
    pub on_actor: ProjectileImpactBehavior,
}

impl Default for Projectile {
    fn default() -> Self {
        Self {
            on_prop: ProjectileImpactBehavior::Die,
            on_actor: ProjectileImpactBehavior::Die,
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Reflect, Debug)]
pub struct Knockback(pub f32);

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
        }
    ).store() + name("Projectile")
}

#[derive(Event, Clone, Copy, PartialEq, Reflect, Debug)]
pub struct KnockbackEvent {
    entity: Entity,
    direction: Vec2,
    force: f32,
}

#[derive(Event, Clone, Copy, PartialEq, Eq, Reflect, Debug)]
pub struct ProjectileImpactEvent {
    pub projectile: Entity,
    pub impacted: Entity,
}

#[derive(Clone, Copy, PartialEq, Eq, Reflect, Debug, Event)]
pub struct ProjectileClashEvent(pub Entity, pub Entity);

#[derive(Clone, Copy, PartialEq, Eq, Reflect, Debug, Default)]
pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<KnockbackEvent>();
        // .add_system(projectile_impact)
        app.add_systems(
            Update,
            (
                tick_lifetimes,
                knockback_events,
                projectile_event_dispatcher,
                kill_projectiles_post_impact,
                knockback_from_projectiles,
            )
                .run_if(in_state(TimerState::Playing)),
        );

        app.add_event::<ProjectileImpactEvent>()
            .add_event::<ProjectileClashEvent>();
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

pub fn projectile_event_dispatcher(
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

fn knockback_from_projectiles(
    mut knockback_events: EventWriter<KnockbackEvent>,
    mut projectile_events: EventReader<ProjectileImpactEvent>,
    projectiles: Query<(&Knockback, Option<&LinearVelocity>)>,
    positions: Query<&Transform>,
) {
    for ProjectileImpactEvent {
        projectile,
        impacted,
    } in projectile_events.read()
    {
        if let Ok((Knockback(knockback), vel)) = projectiles.get(*projectile) {
            let hit_angle = positions.get(*projectile).unwrap().translation
            - positions.get(*impacted).unwrap().translation;
            knockback_events.send(KnockbackEvent {
                entity: *impacted,
                direction: match vel {
                    Some(LinearVelocity (linvel )) => hit_angle.xy() + *linvel,
                    None => hit_angle.xy(),
                },
                force: *knockback,
            });
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

fn knockback_events(
    mut knockback_events: EventReader<KnockbackEvent>,
    mut target_query: Query<&mut ExternalImpulse>,
) {
    for KnockbackEvent {
        entity,
        direction,
        force,
    } in knockback_events.read()
    {
        let impulse_vector = Vec2::normalize(*direction) * *force;
        if let Ok(mut impulse) = target_query.get_mut(*entity) {
            impulse.apply_impulse(impulse_vector);
        }
    }
}
