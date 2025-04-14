use bevy::{core::Name, ecs::system::Res, transform::components::Transform};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree, wrappers::name};
use bevy_stats::Stat;

use crate::{
    action_system::{actions::vel_spawn::vel_spawn, actuator::{Actuator, ActuatorFireStyle}, triggers::{key_action::PlayerActionTrigger, propagation::ParentTrigger}},
    game::stats::ProjectileSpeed,
    twin_stick::{actors::Tracking, ai::keyboard::PlayerAction, player::Cursor, projectile::{projectile, Projectile}}
};

pub fn peashooter(cursor: &Res<Cursor>) -> ComponentTree {
    ((
        PlayerActionTrigger::new([PlayerAction::Shoot1]),
        Tracking(Some(cursor.0)),
        Transform::default()
    ).store()
    + name("Peashooter")) << (
    (
        Name::new("Barrel"),
        Actuator::new ( ActuatorFireStyle::Constantly, 1.3),
        Stat::<ProjectileSpeed>::new(200.),
        Transform::from_xyz(0., 20., 0.),
        ParentTrigger
    ).store()
    + vel_spawn(projectile(0.5, Projectile::default()), 0.)
)
}
