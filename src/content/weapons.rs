use std::sync::Arc;

use bevy::{ecs::system::EntityCommands, prelude::Vec2};
use bevy_composable::{
    app_impl::ComplexSpawnable,
    tree::{ComponentTree, EntityCommandSet},
    CT,
};
use bevy_stats::Stat;
use bevy_twin_stick::{
    bevy_rapier2d::prelude::Velocity,
    projectile::Knockback,
    weapons::{Cooldown, Weapon, WeaponArguments, WeaponFireMode},
};

use crate::{content::shift_pos, game::level::wall};

use super::{projectile::basic_bullet, stats::Damage};

pub fn peashooter() -> ComponentTree {
    return CT!(
        Weapon {
            can_fire: true,
            fire_mode: WeaponFireMode::FullAuto,
            fire_func: Box::new(move |a: &mut WeaponArguments| {
                let parent_transform = a.transforms.get(a.parent).unwrap().clone();
                let cursor_transform = a.transforms.get(a.cursor).unwrap().clone();
                let fire_direction =
                    Vec2::normalize(cursor_transform.translation - parent_transform.translation);
                a.commands.compose(
                    basic_bullet()
                        + CT!(
                            Velocity {
                                linvel: fire_direction * 5000.,
                                angvel: 0.,
                            },
                            Knockback(150.),
                            Stat::<Damage>::new(20.)
                        )
                        + shift_pos(parent_transform.translation + fire_direction * 30.),
                );
            }),
        },
        Cooldown::new(0.3)
    );
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

                    a.commands.compose(wall(
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
