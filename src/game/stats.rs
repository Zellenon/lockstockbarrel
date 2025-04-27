use bevy::{app::App, reflect::Reflect};
use bevy_stats::{RPGResource, RPGStat, StatRegisterable};

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

#[derive(Reflect, Clone, Copy, Debug, Hash)]
pub struct Damage;

#[derive(Reflect, Clone, Copy, Debug, Hash)]
pub struct IdentifyPower;

#[derive(Reflect, Clone, Copy, Debug, Hash)]
pub struct SpotTime;

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

impl RPGStat for Damage {}

impl RPGStat for SpotTime {}

impl RPGStat for IdentifyPower {}

pub(super) fn stats_plugin(app: &mut App) {
    app.register_stat::<MoveSpeed>()
        .register_stat::<Damage>()
        .register_stat::<Knockback>()
        .register_stat::<SpotTime>()
        .register_stat::<IdentifyPower>()
        .register_stat::<Accuracy>()
        .register_stat::<ProjectileSpeed>();

    app.register_resource::<Health>();
}
