use crate::utils::*;
use crate::weapons::{FireWeaponEvent, Weapon, WeaponFireMode};

use bevy::prelude::*;
// use bevy_rapier2d::prelude::*;

#[derive(Component, Resource)]
pub struct MainCamera(pub Entity);

#[derive(Component)]
pub struct ArenaCamera;

#[derive(Component)]
pub struct CursorTracker;

#[derive(Component, Resource)]
pub struct Cursor(pub Entity);

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(player_setup)
            .add_system(update_cursor_tracker);
        app.add_system(fire_player_weapons.run_if(player_exists))
            .add_system(camera_movement.run_if(player_exists));
    }
}

pub fn player_setup(mut commands: Commands) {
    let camera_entity = commands
        .spawn(Camera2dBundle::default())
        .insert(ArenaCamera)
        .id();
    commands.insert_resource(MainCamera(camera_entity));

    let cursor_entity = commands
        .spawn(SpatialBundle::default())
        .insert(CursorTracker)
        .id();
    commands.insert_resource(Cursor(cursor_entity));
}

pub fn update_cursor_tracker(
    mut transforms: Query<&mut Transform>,
    windows: Query<&Window>,
    cam: Res<MainCamera>,
    cursor: Res<Cursor>,
) {
    let camera_transform = transforms.get(cam.0).unwrap().clone();
    let mut cursor_transform = transforms.get_mut(cursor.0).unwrap();
    let window = windows.single();

    if let Some(_position) = window.cursor_position() {
        let new_cursor_pos = screen_to_world(_position, &camera_transform, window);
        cursor_transform.translation = new_cursor_pos.extend(0.);
    }
}

pub fn player_exists(players: Query<(), With<Player>>) -> bool {
    players.iter().count() > 0
}

fn fire_player_weapons(
    buttons: Res<Input<MouseButton>>,
    mut events: EventWriter<FireWeaponEvent>,
    weapons: Query<(Entity, &Weapon)>,
    players_children_query: Query<&Children, With<Player>>,
) {
    for &child in players_children_query.single().iter() {
        if let Ok((entity, weapon)) = weapons.get(child) {
            let trigger_func = weapon.fire_mode;
            if (buttons.just_pressed(MouseButton::Left) && trigger_func == WeaponFireMode::SemiAuto)
                || (buttons.pressed(MouseButton::Left) && trigger_func == WeaponFireMode::FullAuto)
            {
                if weapon.can_fire {
                    events.send(FireWeaponEvent {
                        weapon: entity,
                        target: None,
                    })
                }
            }
        }
    }
}

fn camera_movement(
    cursor: Query<Entity, With<CursorTracker>>,
    player: Query<Entity, With<Player>>,
    camera: Res<MainCamera>,
    mut transforms: Query<&mut Transform>,
) {
    let player_weight = 0.7;
    let delay = 0.15;
    let cursor_loc = transforms.get(cursor.single()).unwrap().translation;
    let player_loc = transforms.get(player.single()).unwrap().translation;
    let mut camera_loc = transforms.get_mut(camera.0).unwrap().translation;
    camera_loc = (cursor_loc * (1. - player_weight) + player_loc * player_weight) * delay
        + camera_loc * (1. - delay);
    transforms.get_mut(camera.0).unwrap().translation = camera_loc;
}
