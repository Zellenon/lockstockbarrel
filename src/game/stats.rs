use bevy::{app::App, reflect::Reflect};
use bevy_stats::{RPGResource, RPGStat};

#[derive(Reflect, Clone, Copy, Debug, Hash)]
pub struct Health;

#[derive(Reflect, Clone, Copy, Debug, Hash)]
pub struct MoveSpeed;

#[derive(Reflect, Clone, Copy, Debug, Hash)]
pub struct Knockback;

#[derive(Reflect, Clone, Copy, Debug, Hash)]
pub struct ProjectileSpeed;

#[derive(Reflect, Clone, Copy, Debug, Hash)]
pub struct Accuracy;

impl RPGStat for Health {
    fn modstyle() -> bevy_stats::ModStyle {
        bevy_stats::ModStyle::MulAdd
    }
}

impl RPGResource for Health {
    fn can_overmax() -> bool {
        true
    }
    fn increase_scaling() -> bevy_stats::resource::ResourceModScaleStyle {
        bevy_stats::resource::ResourceModScaleStyle::SumChange
    }
}

impl RPGStat for MoveSpeed {}

impl RPGStat for Knockback {}

impl RPGStat for ProjectileSpeed {}

impl RPGStat for Accuracy {}

pub(super) fn stats_plugin(app: &mut App) {
    app.register_type::<Health>()
        .register_type::<MoveSpeed>()
        .register_type::<ProjectileSpeed>()
        .register_type::<Accuracy>()
        .register_type::<Knockback>();
}
