use bevy::prelude::Entity;
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree};

use crate::twin_stick::actors::Tracking;

pub(super) fn tracking(e: Entity) -> ComponentTree {
    Tracking(Some(e)).store()
}

pub(super) fn untracked() -> ComponentTree {
    Tracking(None).store()
}
