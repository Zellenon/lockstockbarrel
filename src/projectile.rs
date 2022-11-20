use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::{
    pipeline::CollisionEvent,
    prelude::{Collider, ColliderMassProperties, RigidBody, Velocity},
};

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
        }
    }
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(tick_lifetimes).add_system(projectile_impact);
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
    projectile_query: Query<&Projectile>,
) {
    // println!("boop");
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(e1, e2, _) = collision_event {
            let (projectile, impacted): (&Entity, &Entity) =
                match (projectile_query.get(*e1), projectile_query.get(*e2)) {
                    (Ok(_), Ok(_)) => (e1, e2),
                    (Ok(_), Err(_)) => (e1, e2),
                    (Err(_), Ok(_)) => (e2, e1),
                    (Err(_), Err(_)) => continue,
                };
            commands.entity(*projectile).despawn_recursive();
            // println!("Received collision event: {:?}", collision_event);
        }
    }
}
