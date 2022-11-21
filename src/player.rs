use crate::{
    actors::{Actor, Legs, Tracking},
    stats::{Speed, Stat},
    utils::*,
    weapons::{make_peashooter, FireWeaponEvent, Weapon},
};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Resource)]
pub struct MainCamera(Entity);

#[derive(Component)]
pub struct CursorTracker;

#[derive(Component, Resource)]
pub struct Cursor(Entity);

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(player_setup)
            .add_system(update_cursor_tracker)
            .add_system(keyboard_input_handler)
            .add_system(fire_weapons)
            .add_system(camera_movement);
    }
}

pub fn player_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera_entity = commands.spawn(Camera2dBundle::default()).id();
    commands.insert_resource(MainCamera(camera_entity));

    let cursor_entity = commands
        .spawn(SpatialBundle::default())
        .insert(CursorTracker)
        .id();
    commands.insert_resource(Cursor(cursor_entity));

    commands
        .spawn((
            Player,
            Actor,
            Stat::<Speed>::new(1500.),
            SpatialBundle {
                visibility: Visibility { is_visible: true },
                ..Default::default()
            },
            RigidBody::Dynamic,
            ColliderMassProperties::Density(0.3),
            Velocity::default(),
            Damping {
                linear_damping: 20.,
                angular_damping: 1.0,
            },
            ExternalForce::default(),
            ExternalImpulse::default(),
            Collider::ball(15.),
            LockedAxes::ROTATION_LOCKED,
        ))
        .with_children(|parent| {
            parent.spawn((
                Tracking(Some(cursor_entity)),
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Vec2::new(40., 40.).into(),
                        ..Default::default()
                    },
                    texture: asset_server.load("img/player_head.png"),
                    ..Default::default()
                },
            ));

            parent.spawn((
                Legs::default(),
                Tracking(None),
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Vec2::new(30., 35.).into(),
                        ..Default::default()
                    },
                    texture: asset_server.load("img/player_legs.png"),
                    ..Default::default()
                },
            ));

            parent.spawn(make_peashooter());
        });
}

pub fn update_cursor_tracker(
    mut transforms: Query<&mut Transform>,
    windows: Res<Windows>,
    cam: Res<MainCamera>,
    cursor: Res<Cursor>,
) {
    let camera_transform = transforms.get(cam.0).unwrap().clone();
    let mut cursor_transform = transforms.get_mut(cursor.0).unwrap();
    let window = windows.get_primary().unwrap();

    if let Some(_position) = window.cursor_position() {
        let new_cursor_pos = screen_to_world(_position, &camera_transform, &windows);
        cursor_transform.translation = new_cursor_pos.extend(0.);
    }
}

fn keyboard_input_handler(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut ExternalForce, &Stat<Speed>), With<Player>>,
) {
    let (
        mut force,
        Stat {
            current: speed,
            max: _,
            phantom,
        },
    ) = player_query.single_mut();
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
    let mut force: &mut ExternalForce = &mut force;
    force.force = total_force;
}

fn fire_weapons(
    buttons: Res<Input<MouseButton>>,
    mut events: EventWriter<FireWeaponEvent>,
    weapons: Query<Entity, With<Weapon>>,
    player_query: Query<&Children, With<Player>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        for &child in player_query.single().iter() {
            match weapons.get(child) {
                Ok(entity) => events.send(FireWeaponEvent {
                    weapon: entity,
                    target: None,
                }),
                Err(_) => (),
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
    let drag = 0.15;
    let cursor_loc = transforms.get(cursor.single()).unwrap().translation;
    let player_loc = transforms.get(player.single()).unwrap().translation;
    let mut camera_loc = transforms.get_mut(camera.0).unwrap().translation;
    camera_loc = (cursor_loc * (1. - player_weight) + player_loc * player_weight) * drag
        + camera_loc * (1. - drag);
    transforms.get_mut(camera.0).unwrap().translation = camera_loc;
}
