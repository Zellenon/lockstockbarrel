use bevy::{core::Name, ecs::system::Res, transform::components::Transform};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree, wrappers::name};
use bevy_stats::Stat;

use crate::{
    action_system::{
        actions::vel_spawn::vel_spawn,
        actuator::{Actuator, ActuatorFireStyle},
        triggers::{key_action::PlayerActionTrigger, propagation::ParentTrigger},
    },
    game::stats::{Damage, Knockback, ProjectileSpeed},
    twin_stick::{actors::Tracking, ai::keyboard::PlayerAction, player::Cursor, weapons::Weapon},
};

use super::projectile::{basic_bullet, standard_player_bullet_collision};

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
            Transform::from_xyz(0., 20., 0.),
            ParentTrigger,
            Weapon,
        )
            .store()
            + vel_spawn(basic_bullet() + standard_player_bullet_collision(), 0.))
}
