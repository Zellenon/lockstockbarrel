use bevy::{log::Level, prelude::*};
use bevy_composable::{spawn_complex, ComponentTree};

pub struct LeveleventPlugin;

impl Plugin for LeveleventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelEvent>()
            .add_system(levelevent_to_local_event);
        app.add_event::<SpawnEvent>().add_system(spawn_events);
    }
}

// pub trait LevelEvent: Send + Sync {}
#[derive(Clone)]
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

pub struct SpawnEvent(pub Vec<ComponentTree>);

// impl LevelEvent for SpawnEvent {}

fn spawn_events(mut commands: Commands, mut events: EventReader<SpawnEvent>) {
    for event in events.iter() {
        for component_tree in event.0.iter() {
            spawn_complex(&mut commands, component_tree.clone());
        }
    }
}
