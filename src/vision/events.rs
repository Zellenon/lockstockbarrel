use bevy::{
    app::App,
    ecs::{entity::Entity, event::Event},
    reflect::Reflect,
};

#[derive(Event, Clone, Copy, PartialEq, Reflect, Debug)]
pub struct StartSpottingEvent {
    pub spotter: Entity,
    pub target: Entity,
    pub spot_time: f32,
}

#[derive(Event, Clone, Copy, PartialEq, Reflect, Debug)]
pub struct IdentifyEvent {
    pub identifier: Entity,
    pub target: Entity,
    pub power: f32,
}

#[derive(Event, Clone, Copy, PartialEq, Reflect, Debug)]
pub struct TrackEvent {
    pub tracker: Entity,
    pub target: Entity,
}

#[derive(Event, Clone, Copy, PartialEq, Reflect, Debug)]
pub struct NewTrackEvent {
    pub tracker: Entity,
    pub target: Entity,
}

pub fn event_plugin(app: &mut App) {
    app.register_type::<StartSpottingEvent>()
        .register_type::<IdentifyEvent>()
        .register_type::<TrackEvent>()
        .register_type::<NewTrackEvent>();

    app.add_event::<StartSpottingEvent>()
        .add_event::<IdentifyEvent>()
        .add_event::<TrackEvent>()
        .add_event::<NewTrackEvent>();
}
