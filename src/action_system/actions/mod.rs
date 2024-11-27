use bevy::{prelude::Component, reflect::Reflect};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree};

pub mod kill_self;
pub mod oneshot;
pub mod spawn;

#[derive(Component, Reflect, Debug, Clone, Copy)]
pub struct TelegraphedAction;

pub fn telegraphed() -> ComponentTree {
    TelegraphedAction.store()
}
