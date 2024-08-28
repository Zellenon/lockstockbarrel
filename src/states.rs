use bevy::{app::AppExit, prelude::*, reflect::Reflect};
use bevy_twin_stick::bevy_rapier2d::prelude::RigidBody;

use crate::{
    hud::hud_gui,
    mainmenu::main_menu_gui,
    pause::{pause_gui, pause_on_esc},
};

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_state::<AppState>();
        app.init_state::<GameState>();
        app.init_state::<InGameMenu>();

        app.add_systems(OnEnter(AppState::Exit), exit);
        app.add_systems(OnEnter(AppState::MainMenu), unload_world);

        // app.add_systems(AppState::MainMenu, main_menu_gui);
        app.add_systems(Update, main_menu_gui.run_if(in_state(AppState::MainMenu)));
        app.add_systems(
            Update,
            pause_gui
                .run_if(in_state(InGameMenu::Pause))
                .run_if(in_state(AppState::Game)),
        );
        app.add_systems(
            Update,
            hud_gui
                .run_if(in_state(GameState::PlayingArena))
                .run_if(in_state(AppState::Game)),
        );

        app.add_systems(Update, pause_on_esc.run_if(in_state(AppState::Game)));
    }
}

fn unload_world(mut commands: Commands, gameworld_entities: Query<Entity, With<RigidBody>>) {
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
    PlayingArena,
    #[default]
    Map,
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum InGameMenu {
    #[default]
    None,
    Inventory,
    Pause,
    Options,
}

fn exit(mut app_exit_events: EventWriter<AppExit>) {
    app_exit_events.send(AppExit::Success);
}
