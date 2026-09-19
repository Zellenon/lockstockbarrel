use bevy::{
    ecs::bundle::{self, Bundle},
    prelude::Entity,
};

use crate::twin_stick::actors::Tracking;

pub(super) fn tracking(e: Entity) -> impl Bundle {
    Tracking(Some(e))
}

pub(super) fn untracked() -> impl Bundle {
    Tracking(None)
}
