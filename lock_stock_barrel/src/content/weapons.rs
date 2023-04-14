use std::{sync::Arc, time::Duration};

use bevy::{
    ecs::system::EntityCommands,
    prelude::{Color, Handle, Name, Vec2},
    sprite::{Material2d, Mesh2dHandle},
};
use bevy_composable::{
    app_impl::ComplexSpawnable,
    tree::{ComponentTree, EntityCommandSet},
};
use bevy_mod_transform2d::transform2d::Transform2d;
use bevy_prototype_lyon::{
    prelude::{Fill, GeometryBuilder, Stroke},
    render::ShapeMaterial,
    shapes,
};
use twin_stick::{
    bevy_rapier2d::prelude::{Ccd, Velocity},
    projectile::{
        Damaging, Knockback, Lifespan, Projectile, ProjectileBundle, ProjectileHitBehavior,
        ProjectileImpactBehavior,
    },
    weapons::{Cooldown, Weapon, WeaponArguments, WeaponFireMode},
};

use crate::game::level::wall;

pub fn peashooter() -> ComponentTree {
    let func = move |parent: &mut EntityCommands| {
        parent.insert((
            Weapon {
                can_fire: true,
                fire_mode: WeaponFireMode::FullAuto,
                fire_func: Box::new(move |a: &mut WeaponArguments| {
                    let parent_transform = a.transforms.get(a.parent).unwrap().clone();
                    let cursor_transform = a.transforms.get(a.cursor).unwrap().clone();
                    let fire_direction = Vec2::normalize(
                        cursor_transform.translation - parent_transform.translation,
                    );
                    a.commands.spawn((
                        ProjectileBundle {
                            velocity: Velocity {
                                linvel: fire_direction * 5000.,
                                angvel: 0.,
                            },
                            transform: Transform2d {
                                translation: parent_transform.translation + fire_direction * 30.,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        GeometryBuilder::build_as(&shapes::Circle {
                            radius: 5.,
                            center: Vec2::ZERO,
                        }),
                        Fill::color(Color::YELLOW),
                        Stroke::new(Color::BLACK, 2.0),
                        Knockback(250.),
                        Damaging(20.),
                        Ccd::enabled(),
                        Mesh2dHandle::default(),
                        Handle::<ShapeMaterial>::default(),
                        Name::new("Bullet"),
                    ));
                }),
            },
            Cooldown::new(0.3),
        ));
    };
    (Arc::new(func) as EntityCommandSet).into()
}

pub fn wallgun() -> ComponentTree {
    let func = move |parent: &mut EntityCommands| {
        parent.insert((
            Cooldown::new(0.3),
            Weapon {
                can_fire: true,
                fire_mode: WeaponFireMode::SemiAuto,
                fire_func: Box::new(move |a: &mut WeaponArguments| {
                    let parent_transform = a.transforms.get(a.parent).unwrap().clone();
                    let cursor_transform = a.transforms.get(a.cursor).unwrap().clone();
                    let fire_direction = Vec2::normalize(
                        cursor_transform.translation - parent_transform.translation,
                    );

                    a.commands.spawn_complex(wall(
                        parent_transform.translation.x,
                        parent_transform.translation.y,
                        50.,
                        50.,
                    ));
                }),
            },
        ));
    };
    (Arc::new(func) as EntityCommandSet).into()
}
