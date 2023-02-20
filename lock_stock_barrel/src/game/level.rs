use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_composable::{spawn_complex, ComponentTree, EntityCommandSet};
use bevy_prototype_lyon::prelude as lyon;
use std::sync::Arc;
use twin_stick::bevy_rapier2d::prelude::{Collider, RigidBody};

type LevelMap = Vec<Vec<bool>>;

#[derive(Resource)]
pub struct Level {
    pub arena_map: LevelMap,
    pub resolution: f32,
}

pub fn to_map(map: Vec<Vec<u8>>) -> LevelMap {
    map.iter()
        .map(|w| w.iter().map(|x| *x == 1).collect())
        .collect()
}

pub fn spawn_arena_from_map(mut commands: Commands, level: &Level) {
    let y_len = (level.arena_map.iter().count() as f32) * level.resolution;
    let x_len = (level
        .arena_map
        .iter()
        .map(|w| w.iter().count())
        .max()
        .unwrap() as f32)
        * level.resolution;
    let mut i = 0.;
    for row in level.arena_map.iter() {
        let mut j = 0.;
        for block in row.iter() {
            if *block {
                spawn_complex(
                    &mut commands,
                    wall(
                        j * level.resolution - (x_len * 0.5),
                        (y_len * 0.5) - i * level.resolution,
                        level.resolution,
                        level.resolution,
                    ),
                )
            };
            j = j + 1.;
        }
        i = i + 1.;
    }
}

pub fn wall(x: f32, y: f32, width: f32, height: f32) -> ComponentTree {
    (Arc::new(move |e: &mut EntityCommands| {
        e.insert((
            lyon::GeometryBuilder::build_as(
                &lyon::shapes::Rectangle {
                    extents: Vec2::new(width, height),
                    origin: lyon::shapes::RectangleOrigin::Center,
                },
                lyon::DrawMode::Outlined {
                    fill_mode: lyon::FillMode::color(Color::TEAL),
                    outline_mode: lyon::StrokeMode::color(Color::TEAL),
                },
                Transform::from_xyz(x, y, 0.0),
            ),
            RigidBody::Fixed,
            Collider::cuboid(width / 2., height / 2.),
        ));
    }) as EntityCommandSet)
        .into()
}
