use avian2d::prelude::{FixedJoint, RayCaster, RayHits, RigidBody, SpatialQueryFilter};
use bevy::{
    app::{App, FixedUpdate, Update},
    ecs::{
        component::Component,
        entity::Entity,
        query::{Changed, Or, With},
        system::{Commands, Query},
    },
    hierarchy::{BuildChildren, Children},
    math::{Dir2, Vec2},
    prelude::{ChildBuild, IntoSystemConfigs},
    reflect::Reflect,
    transform::components::Transform,
};
use bevy_stats::{RPGStat, Stat};
use std::f32;

use super::VisionSystems;
use crate::twin_stick::physics::GamePhysicsLayer as GPL;

#[derive(Reflect, Clone, Copy, Debug, Hash)]
pub struct EyeFOV;
#[derive(Reflect, Clone, Copy, Debug, Hash)]
pub struct EyeDistance;

impl RPGStat for EyeFOV {}
impl RPGStat for EyeDistance {}

#[derive(Reflect, Component, Clone, Debug, Default)]
pub struct Eye {
    pub los: Vec<Entity>,
}

#[derive(Reflect, Component, Clone, Copy, Debug, Default)]
pub struct EyeRay;

pub fn eye_plugin(app: &mut App) {
    app.register_type::<Eye>()
        .register_type::<EyeRay>()
        .register_type::<RayHits>();
    app.add_systems(FixedUpdate, (update_eye_targets).in_set(VisionSystems::LoS));
    app.add_systems(Update, (update_eye_children).in_set(VisionSystems::LoS));
}

const MAX_SIGHT_GAP: f32 = 0.5;
pub fn update_eye_children(
    eyes: Query<
        (Entity, Option<&Children>, &Stat<EyeFOV>, &Stat<EyeDistance>),
        (
            With<Eye>,
            Or<(Changed<Stat<EyeFOV>>, Changed<Stat<EyeDistance>>)>,
        ),
    >,
    rays: Query<Entity, (With<RayCaster>, With<EyeRay>)>,
    mut commands: Commands,
) {
    for (e, children, fov, distance) in eyes.iter() {
        println!("b");
        if let Some(children) = children {
            for child in children.iter().filter(|w| rays.get(**w).is_ok()) {
                commands.get_entity(*child).unwrap().despawn();
            }
        }
        let fov = fov.current_value();
        // *5 = * 10. / 2.
        let cone_width = 2. * (fov / 20.).sin() * distance.current_value();
        let n_rays_needed = (cone_width / MAX_SIGHT_GAP) as usize;

        let increment = fov / (n_rays_needed as f32);
        let angles: Vec<_> = (1..n_rays_needed)
            .map(|w| (w as f32 + 0.5) * increment - (fov / 2.) + f32::consts::FRAC_PI_2)
            .collect();
        println!(
            "{:?}, {:?}, {:?}, {:?}, ",
            fov, cone_width, n_rays_needed, angles
        );
        commands.entity(e).with_children(|w| {
            for i in angles {
                w.spawn((
                    EyeRay,
                    RayCaster::new(Vec2::ZERO, Dir2::from_xy_unchecked(i.cos(), i.sin()))
                        .with_max_distance(distance.current_value())
                        .with_query_filter(SpatialQueryFilter::from_mask([
                            GPL::Enemy,
                            GPL::MapSolid,
                            GPL::MapDynamic,
                        ]))
                        .with_max_hits(2),
                    Transform::default(),
                    //RigidBody::Static,
                ));
            }
        });
    }
}

pub fn update_eye_targets(
    mut eyes: Query<(&mut Eye, &Children)>,
    rays: Query<&RayHits, With<EyeRay>>,
) {
    for (mut eye, children) in eyes.iter_mut() {
        eye.los = children
            .iter()
            .filter_map(|w| rays.get(*w).ok())
            .map(|w| w.iter().map(|w| w.entity))
            .flatten()
            .collect();
    }
}
