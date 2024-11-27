use avian2d::prelude::{Collider, LinearVelocity};
use bevy::{app::AppExit, prelude::*, reflect::Reflect};

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_state::<AppState>();
        app.init_state::<GameState>();
        app.init_state::<UIState>();
        app.init_state::<TimerState>();

        app.add_systems(OnEnter(AppState::Exit), exit);
        app.add_systems(OnEnter(AppState::MainMenu), unload_world);

        // app.add_systems(AppState::MainMenu, main_menu_gui);

        app.add_systems(Update, pause_on_esc.run_if(in_state(AppState::Game)));
    }
}

fn unload_world(
    mut commands: Commands,
    gameworld_entities: Query<Entity, Or<(With<LinearVelocity>, With<Collider>)>>,
) {
    for actor in gameworld_entities.into_iter() {
        let mut entity = commands.entity(actor);
        entity.despawn_descendants();
        entity.despawn();
    }
}

#[derive(States, Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum AppState {
    Open,
    Loading,
    #[default]
    MainMenu,
    Game,
    Exit,
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum GameState {
    InLevel,
    #[default]
    OverMap,
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum TimerState {
    #[default]
    Playing,
    Paused,
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum UIState {
    #[default]
    None,
    Inventory,
    Pause,
    Options,
}

pub(crate) fn pause_on_esc(
    input: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<NextState<UIState>>,
) {
    if input.pressed(KeyCode::Escape) {
        state.set(UIState::Pause)
    }
}

fn exit(mut app_exit_events: EventWriter<AppExit>) {
    app_exit_events.send(AppExit::Success);
}
