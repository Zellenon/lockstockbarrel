use crate::{
    actors::{Legs, Tracking},
    utils::*,
};

use bevy::{prelude::*, render::view::visibility};
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct MainCamera(Entity);

#[derive(Component)]
pub struct CursorTracker;
pub struct Cursor(Entity);

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(player_setup)
            .add_system(update_cursor_tracker)
            .add_system(keyboard_input_handler)
            .add_system(camera_movement);
    }
}

pub fn player_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera_entity = commands.spawn_bundle(Camera2dBundle::default()).id();
    commands.insert_resource(MainCamera(camera_entity));

    let cursor_entity = commands
        .spawn_bundle(SpatialBundle::default())
        .insert(CursorTracker)
        .id();
    commands.insert_resource(Cursor(cursor_entity));

    let player = commands
        .spawn() // Player
        .insert_bundle(SpatialBundle {
            visibility: Visibility { is_visible: true },
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(ColliderMassProperties::Density(0.3))
        .insert(Velocity::default())
        .insert(Damping {
            linear_damping: 20.,
            angular_damping: 1.0,
        })
        .insert(ExternalForce::default())
        .insert(Player)
        .insert(Collider::ball(15.))
        .insert(LockedAxes::ROTATION_LOCKED)
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Vec2::new(40., 40.).into(),
                        ..Default::default()
                    },
                    texture: asset_server.load("img/player_head.png"),
                    ..Default::default()
                })
                .insert_bundle(SpatialBundle {
                    visibility: Visibility { is_visible: true },
                    ..Default::default()
                })
                .insert(Tracking(Some(cursor_entity)));

            parent
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Vec2::new(30., 35.).into(),
                        ..Default::default()
                    },
                    texture: asset_server.load("img/player_legs.png"),
                    ..Default::default()
                })
                .insert(Tracking(None))
                .insert_bundle(SpatialBundle {
                    visibility: Visibility { is_visible: true },
                    transform: Transform::from_xyz(0., 0., -1.),
                    ..Default::default()
                })
                .insert(Legs::default());
        });
}

pub fn update_cursor_tracker(
    mut transforms: Query<&mut Transform>,
    mut events: EventReader<CursorMoved>,
    windows: Res<Windows>,
    cam: Res<MainCamera>,
    cursor: Res<Cursor>,
) {
    let camera_transform = transforms.get(cam.0).unwrap().clone();
    let mut cursor_transform = transforms.get_mut(cursor.0).unwrap();
    for event in events.iter() {
        let new_cursor_pos = screen_to_world(event.position, &camera_transform, &windows);
        cursor_transform.translation = new_cursor_pos.extend(0.);
    }
}

fn keyboard_input_handler(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<&mut ExternalForce, With<Player>>,
) {
    let speed = 1500.;
    let mut total_force = Vec2::new(0., 0.);
    if keyboard_input.pressed(KeyCode::A) {
        total_force.x += -speed;
    }
    if keyboard_input.pressed(KeyCode::D) {
        total_force.x += speed;
    }
    if keyboard_input.pressed(KeyCode::W) {
        total_force.y += speed;
    }
    if keyboard_input.pressed(KeyCode::S) {
        total_force.y += -speed;
    }
    let mut force: &mut ExternalForce = &mut *player.get_single_mut().unwrap();
    force.force = total_force;
}

fn camera_movement(
    cursor: Query<Entity, With<CursorTracker>>,
    player: Query<Entity, With<Player>>,
    camera: Res<MainCamera>,
    mut transforms: Query<&mut Transform>,
) {
    let player_weight = 0.7;
    let drag = 0.15;
    let cursor_loc = transforms.get(cursor.single()).unwrap().translation;
    let player_loc = transforms.get(player.single()).unwrap().translation;
    let mut camera_loc = transforms.get_mut(camera.0).unwrap().translation;
    camera_loc = (cursor_loc * (1. - player_weight) + player_loc * player_weight) * drag
        + camera_loc * (1. - drag);
    transforms.get_mut(camera.0).unwrap().translation = camera_loc;
}
