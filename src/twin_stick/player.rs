use bevy::prelude::{Reflect, SpatialBundle, Transform};

use bevy::{
    prelude::{
        in_state, App, Camera2dBundle, Commands, Component, Entity, IntoSystemConfigs, Name, Query,
        Res, Resource, Startup, Update, With,
    },
    window::Window,
};

use crate::{states::TimerState, transform2d::Transform2d};

use super::utils::screen_to_world;

#[derive(Component, Resource, Clone, Copy, PartialEq, Eq, Reflect, Debug)]
pub struct MainCamera(pub Entity);

#[derive(Component, Clone, Copy, PartialEq, Eq, Reflect, Debug)]
pub struct TwinStickCamera;

#[derive(Component, Clone, Copy, PartialEq, Eq, Reflect, Debug)]
pub struct CursorTracker;

#[derive(Component, Resource, Clone, Copy, PartialEq, Eq, Reflect, Debug)]
pub struct Cursor(pub Entity);

#[derive(Component, Clone, Copy, PartialEq, Eq, Reflect, Debug)]
pub struct Player;

pub(super) fn player_plugin(app: &mut App) {
    app.add_systems(Startup, player_setup);
    app.add_systems(
        Update,
        (
            update_cursor_tracker,
            // fire_player_weapons.run_if(player_exists),
        )
            .run_if(in_state(TimerState::Playing)),
    );
}

pub fn player_setup(mut commands: Commands) {
    let camera_entity = commands
        .spawn(Camera2dBundle::default())
        .insert(Name::new("Twin-Stick Player Camera"))
        .insert(Transform::default().with_layer(100.))
        .insert(TwinStickCamera)
        .id();
    commands.insert_resource(MainCamera(camera_entity));

    let cursor_entity = commands
        .spawn(SpatialBundle::default())
        .insert(Name::new("Cursor"))
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

// pub fn fire_player_weapons(
//     buttons: Res<ButtonInput<MouseButton>>,
//     mut events: EventWriter<FireWeaponEvent>,
//     weapons: Query<(Entity, &Weapon)>,
//     players_children_query: Query<&Children, With<Player>>,
// ) {
//     for parent_player in players_children_query.iter() {
//         for &child in parent_player.iter() {
//             if let Ok((entity, weapon)) = weapons.get(child) {
//                 let trigger_func = weapon.fire_mode;
//                 if ((buttons.just_pressed(MouseButton::Left)
//                     && trigger_func == WeaponFireMode::SemiAuto)
//                     || (buttons.pressed(MouseButton::Left)
//                         && trigger_func == WeaponFireMode::FullAuto))
//                     && weapon.can_fire
//                 {
//                     events.send(FireWeaponEvent {
//                         weapon: entity,
//                         target: None,
//                     });
//                 }
//             }
//         }
//     }
// }
