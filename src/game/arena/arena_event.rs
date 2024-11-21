use bevy::prelude::*;
use bevy_composable::{app_impl::ComplexSpawnable, tree::ComponentTree};

pub struct ArenaEventPlugin;

impl Plugin for ArenaEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ArenaEvent>()
            .add_systems(Update, levelevent_to_local_event);
        app.add_event::<SpawnEvent>();
        app.add_systems(Update, spawn_events);
    }
}

// pub trait ArenaEvent: Send + Sync {}
#[derive(Clone, Event)]
pub enum ArenaEvent {
    Spawn(Vec<ComponentTree>),
}

fn levelevent_to_local_event(
    mut levelevents: EventReader<ArenaEvent>,
    mut spawn_events: EventWriter<SpawnEvent>,
) {
    for levelevent in levelevents.read() {
        match levelevent {
            ArenaEvent::Spawn(wave) => spawn_events.send(SpawnEvent(wave.to_vec())),
        };
    }
}
