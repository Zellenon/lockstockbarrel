use avian2d::prelude::Collider;
use bevy::{
    color::{palettes::css::RED, Color},
    core::Name,
    ecs::system::Res,
    math::Vec2,
    sprite::Sprite,
    transform::components::Transform,
    utils::default,
};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree, wrappers::name};
use bevy_stats::Stat;

use crate::{
    action_system::{
        actions::vel_spawn::vel_spawn,
        actuator::{Actuator, ActuatorFireStyle},
        triggers::{key_action::PlayerActionTrigger, propagation::ParentTrigger},
    },
    game::stats::{
        Accuracy, Damage, IdentifyPower, Knockback, ProjectileSpeed, ShotCount, SpotTime,
    },
    twin_stick::{
        actors::Tracking,
        ai::keyboard::PlayerAction,
        player::Cursor,
        projectile::{projectile, Projectile},
        weapons::{SpreadType, Weapon},
    },
    vision::tracking::TrackAttack,
};

use super::projectile::{self, basic_bullet, standard_player_bullet_collision};

pub fn peashooter(cursor: &Res<Cursor>) -> ComponentTree {
    ((
        PlayerActionTrigger::new([PlayerAction::Shoot1]),
        Tracking(Some(cursor.0)),
        Transform::default(),
    )
        .store()
        + name("Peashooter"))
        << ((
            Name::new("Barrel"),
            Actuator::new(ActuatorFireStyle::SemiAuto(false), 1.3),
            Stat::<ProjectileSpeed>::new(200.),
            Stat::<Damage>::new(2.),
            Stat::<Knockback>::new(30.),
            Stat::<ShotCount>::new(1.),
            Transform::from_xyz(0., 20., 0.),
            ParentTrigger,
            Weapon::default(),
        )
            .store()
            + vel_spawn(
                basic_bullet() + standard_player_bullet_collision(),
                0.,
                true,
            ))
}

pub fn sonar_launcher(cursor: &Res<Cursor>) -> ComponentTree {
    ((
        PlayerActionTrigger::new([PlayerAction::Shoot3]),
        Tracking(Some(cursor.0)),
        Transform::default(),
    )
        .store()
        + name("Sonar"))
        << ((
            Name::new("Barrel"),
            Actuator::new(ActuatorFireStyle::SemiAuto(false), 1.3),
            Stat::<ProjectileSpeed>::new(20.),
            Stat::<IdentifyPower>::new(27.0),
            Stat::<SpotTime>::new(5.0),
            Stat::<ShotCount>::new(5.),
            Stat::<Accuracy>::new(50.),
            SpreadType::Spaced,
            Transform::default(),
            ParentTrigger,
            Weapon::default(),
        )
            .store()
            + vel_spawn(
                projectile(5., Projectile::default())
                    + standard_player_bullet_collision()
                    + (
                        Collider::circle(20.),
                        Sprite {
                            color: Color::Srgba(RED),
                            custom_size: Some(Vec2::new(40., 40.)),
                            ..default()
                        },
                    )
                        .store(),
                0.,
                true,
            ))
}
