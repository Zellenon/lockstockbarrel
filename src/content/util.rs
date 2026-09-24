use bevy::{
    ecs::bundle::{self, Bundle},
    prelude::Entity,
};

use crate::{twin_stick::actors::Tracking, util::spawning::Spawnable};

pub(super) fn tracking(e: Entity) -> impl Spawnable {
    Tracking(Some(e))
}

pub(super) fn untracked() -> impl Spawnable {
    Tracking(None)
}
