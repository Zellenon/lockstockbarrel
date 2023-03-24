use bevy::{app::AppExit, prelude::*};
use twin_stick::bevy_rapier2d::prelude::RigidBody;

use crate::{
    hud::hud_gui,
    mainmenu::main_menu_gui,
    pause::{pause_gui, pause_on_esc},
};

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<AppState>();
        app.add_state::<GameState>();
        app.add_state::<InGameMenu>();

        app.add_system(exit.in_schedule(OnEnter(AppState::Exit)));
        app.add_system(unload_world.in_schedule(OnEnter(AppState::MainMenu)));
        app.add_system(main_menu_gui.in_set(OnUpdate(AppState::MainMenu)));
        app.add_system(
            pause_gui
                .run_if(in_state(AppState::Game))
                .run_if(in_state(InGameMenu::Pause)),
        );
        app.add_system(
            hud_gui
                .run_if(in_state(AppState::Game))
                .run_if(in_state(GameState::PlayingArena)),
        );

        app.add_system(pause_on_esc.in_set(OnUpdate(AppState::Game)));
    }
}

fn unload_world(mut commands: Commands, gameworld_entities: Query<Entity, With<RigidBody>>) {
    for actor in gameworld_entities.into_iter() {
        let mut entity = commands.entity(actor);
        entity.despawn_descendants();
        entity.despawn();
    }
}

#[derive(States, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    Open,
    Loading,
    #[default]
    MainMenu,
    Game,
    Exit,
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    PlayingArena,
    #[default]
    Map,
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InGameMenu {
    #[default]
    None,
    Inventory,
    Pause,
    Options,
}

fn exit(mut app_exit_events: EventWriter<AppExit>) {
    app_exit_events.send(AppExit);
}
