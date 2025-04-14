use std::f32;

use avian2d::prelude::ExternalImpulse;
use bevy::{
    app::App, math::{Quat, Vec2}, prelude::{Commands, Component, Query, Transform, Trigger}, reflect::Reflect, transform::components::GlobalTransform
};
use bevy_composable::{
    app_impl::{ComplexSpawnable, ComponentTreeable},
    tree::ComponentTree,
};
use bevy_stats::Stat;

use crate::{action_system::actuator::Actuate, game::stats::{Accuracy, ProjectileSpeed}, transform2d::To2D, util::add_observer_to_component};

#[derive(Clone, Copy, Debug, Reflect, PartialEq)]
pub struct AngleOffset(pub Vec2);

impl AngleOffset {
    pub fn new(vec: Vec2) -> Self {
        Self(vec.normalize())
    }
}

#[derive(Component, Clone)]
pub struct VelSpawnAction {
    pub payload: Vec<(ComponentTree, AngleOffset)>,
}

impl VelSpawnAction {
    pub fn spawn<T: Into<AngleOffset>>(tree: ComponentTree, angle: T) -> Self {
        Self {
            payload: vec![(tree, angle.into())],
        }
    }

    pub fn spawns<A: Into<AngleOffset>, T: Iterator<Item = (ComponentTree, A)>>(trees: T) -> Self {
        Self {
            payload: trees.map(|w| (w.0, w.1.into())).collect(),
        }
    }

    pub fn setup(app: &mut App) {
        // app.register_type::<SpawnAction>();
        app.add_observer(add_observer_to_component::<Self, _, _, _, _>(
            do_vel_spawn_action,
        ));
    }
}

pub fn vel_spawn<T: Into<AngleOffset>>(tree: ComponentTree, angle: T) -> ComponentTree {
    VelSpawnAction::spawn(tree, angle).store()
}

pub fn vel_spawns<A: Into<AngleOffset>, T: Iterator<Item = (ComponentTree, A)>>(trees: T) -> ComponentTree {
    VelSpawnAction::spawns(trees).store()
}

pub fn do_vel_spawn_action(
    trigger: Trigger<Actuate>,
    spawners: Query<(&VelSpawnAction, &GlobalTransform, Option<&Stat<ProjectileSpeed>>, Option<&Stat<Accuracy>>)>,
    mut commands: Commands,
) {
    if let Ok((spawn_action, transform, speed, accuracy)) = spawners.get(trigger.entity()) {
        for (payload, angle_offset) in spawn_action.payload.iter() {
            let (scale, rotation, translation) = transform.to_scale_rotation_translation();
            let spawned_transform = Transform {
                translation,
                rotation: rotation * Quat::from_2d(angle_offset.0.to_angle()),
                scale
            };

            commands.compose(
                payload.clone() + (
                    spawned_transform,
                    ExternalImpulse::new(
                        Vec2::from_angle(rotation.to_2d() + angle_offset.0.to_angle() + f32::consts::FRAC_PI_2)
                        * speed.map(|w| w.current_value()).unwrap_or(10.0)
                    )
                ).store()
            );
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
