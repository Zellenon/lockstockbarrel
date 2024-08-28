use bevy::prelude::*;

use super::level_event::LevelEvent;
use crate::content::{enemies::basic_walker, shift_pos};

pub struct LeveleventManagerPlugin;

impl Plugin for LeveleventManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WakeLEManager>()
            .add_event::<FireLEManager>();

        app.add_systems(
            Update,
            (
                activate_levelevent_timer,
                activate_child_managers,
                tick_basic_timers,
                fire_lemanagers,
            ),
        );
    }
}

#[derive(Component)]
pub struct LeveleventManager {
    // pub activation_condition: &'static System,
    // pub priming_function: Option<Box<dyn System>>,
    // pub activation_condition: Box<dyn LevelEvent + Send + Sync>,
    pub state: LEManagerState,
    pub fire_event: LevelEvent,
    // reset_condition: None | Trigger<T>
    // Children
}

impl LeveleventManager {
    pub fn new(fire_event: LevelEvent) -> Self {
        Self {
            state: LEManagerState::Off,
            fire_event,
        }
    }
}

#[derive(Event)]
struct WakeLEManager(Entity);
#[derive(Event)]
struct FireLEManager(Entity);

#[derive(Eq, PartialEq)]
pub enum LEManagerState {
    Off,
    Live,
    Fired,
}

#[derive(Component)]
pub struct BasicTimer(Timer);

pub fn test_lemanager_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let head_tex = asset_server.load("img/placeholder_head.png");
    let leg_tex = asset_server.load("img/placeholder_legs.png");
    let spawn_event = LevelEvent::Spawn(vec![
        basic_walker(head_tex.clone(), leg_tex.clone()) + shift_pos((500., 0.)),
        basic_walker(head_tex.clone(), leg_tex.clone()) + shift_pos((0., 500.)),
        basic_walker(head_tex.clone(), leg_tex.clone()) + shift_pos((-500., 0.)),
        basic_walker(head_tex.clone(), leg_tex.clone()) + shift_pos((500., 0.)),
        basic_walker(head_tex.clone(), leg_tex.clone()) + shift_pos((0., 500.)),
        basic_walker(head_tex.clone(), leg_tex.clone()) + shift_pos((-500., 0.)),
    ]);
    commands.spawn(LeveleventManager::new(spawn_event));
}

fn activate_child_managers(
    manager_query: Query<(Entity, Option<&Parent>, &LeveleventManager)>,
    mut wake_events: EventWriter<WakeLEManager>,
) {
    for (entity, parent, manager) in manager_query.iter() {
        if (parent == None) && (manager.state == LEManagerState::Off) {
            wake_events.send(WakeLEManager(entity));
        }
    }
}

fn activate_levelevent_timer(
    mut commands: Commands,
    mut priming_managers: Query<&mut LeveleventManager>,
    mut wake_events: EventReader<WakeLEManager>,
) {
    for event in wake_events.read() {
        if let Ok(mut manager) = priming_managers.get_mut(event.0) {
            commands
                .entity(event.0)
                .insert(BasicTimer(Timer::from_seconds(2., TimerMode::Once)));
            manager.state = LEManagerState::Live;
        }
    }
}

fn tick_basic_timers(mut timers: Query<&mut BasicTimer>, time: Res<Time>) {
    for mut timer in timers.iter_mut() {
        timer.0.tick(time.delta());
    }
}

fn fire_lemanagers(
    mut timers: Query<(&BasicTimer, Entity, &mut LeveleventManager)>,
    mut events: EventWriter<LevelEvent>,
) {
    for (timer, _, mut manager) in timers.iter_mut() {
        if (*timer).0.finished() && manager.state == LEManagerState::Live {
            manager.state = LEManagerState::Fired;
            events.send(manager.fire_event.clone());
            // fire_events.send(FireLEManager(entity))
        }
    }
}
