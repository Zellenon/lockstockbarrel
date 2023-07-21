use bevy::prelude::*;
use bevy_composable::{app_impl::ComplexSpawnable, tree::ComponentTree};

pub struct LeveleventPlugin;

impl Plugin for LeveleventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelEvent>()
            .add_systems(Update, levelevent_to_local_event);
        app.add_event::<SpawnEvent>();
        app.add_systems(Update, spawn_events);
    }
}

// pub trait LevelEvent: Send + Sync {}
#[derive(Clone, Event)]
pub enum LevelEvent {
    Spawn(Vec<ComponentTree>),
}

fn levelevent_to_local_event(
    mut levelevents: EventReader<LevelEvent>,
    mut spawn_events: EventWriter<SpawnEvent>,
) {
    for levelevent in levelevents.iter() {
        match levelevent {
            LevelEvent::Spawn(wave) => spawn_events.send(SpawnEvent(wave.to_vec())),
        }
    }
}

// -------------------------------------

#[derive(Clone, Event)]
pub struct SpawnEvent(pub Vec<ComponentTree>);

// impl LevelEvent for SpawnEvent {}

fn spawn_events(mut commands: Commands, mut events: EventReader<SpawnEvent>) {
    for event in events.iter() {
        for component_tree in event.0.iter() {
            commands.spawn_complex(component_tree.clone());
        }
    }
}
