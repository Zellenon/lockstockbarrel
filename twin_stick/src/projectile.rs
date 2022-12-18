use std::{marker::PhantomData, time::Duration};

use bevy::prelude::*;
use bevy_rapier2d::{
    pipeline::CollisionEvent,
    prelude::{
        ActiveEvents, Collider, ColliderMassProperties, ExternalImpulse, RigidBody, Velocity,
    },
};

use bevy_stats::{Health, Stat, StatChangeEvent};

#[derive(Component)]
pub struct Lifespan(Timer);

impl Default for Lifespan {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(400), TimerMode::Once))
    }
}

#[derive(Component)]
pub struct Projectile {
    pub on_hit: ProjectileHitBehavior,
    pub on_impact: ProjectileImpactBehavior,
}

impl Default for Projectile {
    fn default() -> Self {
        Self {
            on_hit: ProjectileHitBehavior::Die,
            on_impact: ProjectileImpactBehavior::Die,
        }
    }
}

pub enum ProjectileHitBehavior {
    Die,
    Bounce,
}

pub enum ProjectileImpactBehavior {
    Die,
    Bounce,
}

#[derive(Component)]
pub struct Knockback(pub f32);

#[derive(Component)]
pub struct Damaging(pub f32);

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub projectile: Projectile,
    pub visibility: Visibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub rigidbody: RigidBody,
    pub velocity: Velocity,
    pub mass_properties: ColliderMassProperties,
    pub collider: Collider,
    pub event_trigger: ActiveEvents,
}

impl Default for ProjectileBundle {
    fn default() -> Self {
        Self {
            projectile: Projectile::default(),
            visibility: Visibility { is_visible: true },
            velocity: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            rigidbody: RigidBody::Dynamic,
            mass_properties: ColliderMassProperties::Density(1.),
            collider: Collider::ball(5.),
            event_trigger: ActiveEvents::COLLISION_EVENTS,
        }
    }
}

pub struct KnockbackEvent {
    entity: Entity,
    direction: Vec2,
    force: f32,
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<KnockbackEvent>()
            .add_system(tick_lifetimes)
            .add_system(knockback_events)
            .add_system(projectile_impact);
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

fn projectile_impact(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    projectile_query: Query<(
        &Projectile,
        &Velocity,
        Option<&Damaging>,
        Option<&Knockback>,
    )>,
    target_query: Query<(&RigidBody, Option<&ExternalImpulse>, Option<&Stat<Health>>)>,
    mut knockback_events: EventWriter<KnockbackEvent>,
    mut health_events: EventWriter<StatChangeEvent<Health>>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(e1, e2, _) = collision_event {
            let (projectile, target): (&Entity, &Entity) =
                match (projectile_query.get(*e1), projectile_query.get(*e2)) {
                    (Ok(_), _) => (e1, e2),
                    (Err(_), Ok(_)) => (e2, e1),
                    (Err(_), Err(_)) => continue,
                };

            let projectile_data = projectile_query.get(*projectile).unwrap();
            let target_data = target_query.get(*target).unwrap();
            if let Some(Knockback(force)) = projectile_data.3 {
                if let Some(_) = target_data.1 {
                    knockback_events.send(KnockbackEvent {
                        entity: *target,
                        direction: projectile_data.1.linvel,
                        force: *force,
                    })
                }
            }
            if let Some(Damaging(damage)) = projectile_data.2 {
                if let Some(_) = target_data.2 {
                    health_events.send(StatChangeEvent {
                        target: *target,
                        amount: damage * -1.,
                        phantom: PhantomData,
                    })
                }
            }
            commands.entity(*projectile).despawn_recursive();
            // println!("Received collision event: {:?}", collision_event);
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
    } in knockback_events.iter()
    {
        let impulse_vector = Vec2::normalize(*direction) * *force;
        let mut impulse = target_query.get_mut(*entity).unwrap();
        impulse.impulse = impulse_vector;
    }
}
