use avian2d::prelude::LinearVelocity;
use bevy::{
    app::App,
    ecs::{
        bundle::Bundle,
        entity::Entity,
        hierarchy::ChildOf,
        observer::On,
        query::{Or, With},
    },
    math::{Quat, Vec2},
    prelude::{Commands, Component, Query, Transform},
    reflect::Reflect,
    transform::components::GlobalTransform,
};
use bevy_stats::Stat;
use rand;
use rand::RngExt;
use std::{f32, sync::Arc};

use crate::{
    action_system::actuator::Actuate,
    game::stats::{Accuracy, ProjectileSpeed, ShotCount},
    transform2d::To2D,
    twin_stick::{
        actors::Actor,
        weapons::{SpreadType, Weapon},
    },
    util::{
        add_observer_to_component,
        spawning::{store, StoredCommand},
    },
};

use super::spawn::SpawnedBy;

#[derive(Clone, Copy, Debug, Reflect, PartialEq)]
pub struct AngleOffset(pub Vec2);

impl AngleOffset {
    pub fn new(vec: Vec2) -> Self {
        Self(vec.normalize())
    }
}

#[derive(Component, Clone)]
pub struct VelSpawnAction {
    pub payload: Vec<(StoredCommand, AngleOffset, bool)>,
}

impl VelSpawnAction {
    pub fn new<T: Into<AngleOffset>>(bundle: impl Bundle, angle: T, uses_count: bool) -> Self {
        Self {
            payload: vec![(store(bundle), angle.into(), uses_count)],
        }
    }

    pub fn spawns<A: Into<AngleOffset>, T: Iterator<Item = (impl Bundle, A, bool)>>(
        trees: T,
    ) -> Self {
        Self {
            payload: trees.map(|w| (store(w.0), w.1.into(), w.2)).collect(),
        }
    }

    pub fn setup(app: &mut App) {
        // app.register_type::<SpawnAction>();
        app.add_observer(add_observer_to_component::<Self, _, _, _, _>(
            do_vel_spawn_action,
        ));
    }
}

pub fn vel_spawn<T: Into<AngleOffset>>(
    bundle: impl Bundle,
    angle: T,
    uses_count: bool,
) -> impl Bundle {
    VelSpawnAction {
        payload: vec![(store(bundle), angle.into(), uses_count)],
    }
}

pub fn vel_spawns<A: Into<AngleOffset>, T: Iterator<Item = (impl Bundle, A, bool)>>(
    bundles: T,
) -> impl Bundle {
    VelSpawnAction::spawns(bundles)
}

pub fn do_vel_spawn_action(
    trigger: On<Actuate>,
    spawners: Query<(
        Entity,
        &VelSpawnAction,
        &GlobalTransform,
        Option<&Stat<ProjectileSpeed>>,
        Option<&Stat<Accuracy>>,
        Option<&SpreadType>,
        Option<&Stat<ShotCount>>,
    )>,
    attackers: Query<Entity, Or<(With<Actor>, With<Weapon>)>>,
    parents: Query<&ChildOf>,
    mut commands: Commands,
) {
    if let Ok((e, spawn_action, transform, speed, accuracy, spread, shot_count)) =
        spawners.get(trigger.event().0)
    {
        let fire_cone = f32::consts::PI * accuracy.map(|w| w.current_value()).unwrap_or(0.) / 100.;
        let shot_count = shot_count.map(|w| w.current_value() as usize).unwrap_or(0);
        let spawn_angles: Vec<_> = match spread.unwrap_or(&SpreadType::default()) {
            SpreadType::Spaced => {
                let increment = fire_cone / (shot_count as f32);
                (0..shot_count)
                    .map(|w| (w as f32 + 0.5) * increment - (fire_cone / 2.))
                    .collect()
            }
            SpreadType::NormalDistribution => todo!(),
            SpreadType::Jittered => todo!(),
            SpreadType::TrueRandom => (0..shot_count)
                .map(|_| {
                    rand::rng().sample::<f32, _>(rand::distr::StandardUniform) * (fire_cone / 2.)
                })
                .collect(),
        };
        print!("{:?}", spawn_angles);
        for (payload, angle_offset, uses_count) in spawn_action.payload.iter() {
            let (scale, rotation, translation) = transform.to_scale_rotation_translation();
            let spawned_transform = Transform {
                translation,
                rotation: rotation * Quat::from_2d(angle_offset.0.to_angle()),
                scale,
            };

            let count = {
                if *uses_count {
                    shot_count
                } else {
                    0
                }
            };
            for i in 0..count {
                // If there's a first ancestor with Weapon/Actor
                payload({
                    let new_entity = &mut commands.spawn((
                        spawned_transform,
                        LinearVelocity(
                            Vec2::from_angle(
                                rotation.to_2d()
                                    + angle_offset.0.to_angle()
                                    + f32::consts::FRAC_PI_2
                                    + spawn_angles.get(i).unwrap(),
                            ) * speed.map(|w| w.current_value()).unwrap_or(10.0),
                        ),
                    ));
                    if let Some(attacker) = std::iter::once(e)
                        .chain(parents.iter_ancestors(e))
                        .filter(|w| attackers.get(*w).is_ok())
                        .next()
                    {
                        new_entity.insert(SpawnedBy(attacker));
                    } else {
                    }
                    new_entity
                });
            }
        }
    }
}

impl Into<AngleOffset> for Vec2 {
    fn into(self) -> AngleOffset {
        AngleOffset(self)
    }
}

impl Into<AngleOffset> for f32 {
    fn into(self) -> AngleOffset {
        AngleOffset(Vec2::from_angle(self))
    }
}

impl Into<AngleOffset> for Quat {
    fn into(self) -> AngleOffset {
        self.to_2d().into()
    }
}
