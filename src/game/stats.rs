use bevy::{app::App, reflect::Reflect};
use bevy_stats::{RPGResource, RPGStat};

#[derive(Reflect, Clone, Copy)]
pub struct Health;
#[derive(Reflect, Clone, Copy)]
pub struct MoveSpeed;
#[derive(Reflect, Clone, Copy)]
pub struct Knockback;

pub(super) fn stats_plugin(app: &mut App) {
    app.register_type::<Health>()
        .register_type::<MoveSpeed>()
        .register_type::<Knockback>();
}

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
        bevy_stats::resource::ResourceModScaleStyle::Percentage
    }
}

impl RPGStat for MoveSpeed {}

impl RPGStat for Knockback {}
