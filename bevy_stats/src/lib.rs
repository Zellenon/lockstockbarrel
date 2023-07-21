use bevy::prelude::*;

pub use resource::{RPGResource, Resource};
pub use stat::{RPGStat, Stat};
pub use statmod::DeleteStatMod;
use systems::delete_stat_mod;
pub mod resource;
pub mod stat;
pub mod statmod;
pub mod systems;

pub struct StatPlugin;

impl Plugin for StatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DeleteStatMod>();
        app.add_systems(Update, delete_stat_mod); //TODO: See if Update is the right time to be
                                                  //running this
    }
}

// fn do_stat_change<T>(
//     mut change_events: EventReader<StatChangeEvent<T>>,
//     mut targets: Query<&mut Stat<T>>,
// ) where
//     T: RPGStat,
// {
//     for StatChangeEvent {
//         target,
//         amount,
//         phantom: _,
//     } in change_events.iter()
//     {
//         let mut target_stat = targets.get_mut(*target).unwrap();
//         target_stat.current = target_stat.current_value() + amount;
//     }
// }
