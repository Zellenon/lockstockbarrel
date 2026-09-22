use bevy::{
    app::Plugin,
    ecs::{event::Trigger, lifecycle::Add, observer::On, system::IntoObserverSystem},
    prelude::{Bundle, Commands, Component, EntityEvent, Message},
};
use gimmie::give_images;

pub struct UtilPlugin;

pub mod gimmie;
pub mod spawning;

impl Plugin for UtilPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_observer(give_images);
    }
}

/// Add the provided function to all entities with component T.
pub fn add_observer_to_component<T, S, E, B, M>(
    observer_function: S,
) -> impl FnMut(On<Add, T>, Commands) -> ()
where
    T: Component,
    B: Bundle,
    E: EntityEvent + 'static,
    S: IntoObserverSystem<E, B, M> + Send + Sync + Clone,
{
    move |trigger: On<Add, T>, mut commands: Commands| {
        commands
            .entity(trigger.event().entity)
            .observe(observer_function.clone());
    }
}
