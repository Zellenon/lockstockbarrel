use bevy::prelude::*;

struct LevelEventManager {}

#[derive(Component)]
struct LinearLeveleventManager {
    activation_condition: &'static System,
    // Children
}
pub enum LEManagerState {
    Off,
    Live,
    Fired,
}
