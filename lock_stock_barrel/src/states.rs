use bevy::{
    app::AppExit,
    prelude::{
        Commands, DespawnRecursiveExt, Entity, EventWriter, Input, KeyCode, Plugin, Query, Res,
        SystemSet, With,
    },
};
use iyes_loopless::prelude::*;
use twin_stick::{actors::Actor, bevy_rapier2d::prelude::RigidBody};

use crate::{
    mainmenu::main_menu_gui,
    pause::{pause_gui, pause_on_esc},
};

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_loopless_state(AppState::MainMenu);
        app.add_loopless_state(GameState::PlayingArena);
        app.add_loopless_state(InGameMenu::None);

        app.add_system(exit.run_in_state(AppState::Exit));
        app.add_enter_system(AppState::MainMenu, unload_world);
        app.add_system(main_menu_gui.run_in_state(AppState::MainMenu));
        app.add_system(
            pause_gui
                .run_in_state(AppState::Game)
                .run_in_state(InGameMenu::Pause),
        );

        app.add_system(pause_on_esc.run_in_state(AppState::Game));
    }
}

fn unload_world(mut commands: Commands, gameworld_entities: Query<Entity, With<RigidBody>>) {
    for actor in gameworld_entities.into_iter() {
        let mut entity = commands.entity(actor);
        entity.despawn_descendants();
        entity.despawn();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    MainMenu,
    Game,
    Exit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    PlayingArena,
    Map,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InGameMenu {
    None,
    Inventory,
    Pause,
    Options,
}

fn exit(mut app_exit_events: EventWriter<AppExit>) {
    app_exit_events.send(AppExit);
}
