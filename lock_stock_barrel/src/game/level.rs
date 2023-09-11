use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_composable::{
    app_impl::ComplexSpawnable,
    tree::{ComponentTree, EntityCommandSet},
};
use bevy_prototype_lyon::prelude::{self as lyon, Fill, ShapeBundle};
use std::sync::Arc;
use twin_stick::bevy_mod_transform2d::transform2d::Transform2d;
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
                commands.spawn_complex(wall(
                    j * level.resolution - (x_len * 0.5),
                    (y_len * 0.5) - i * level.resolution,
                    level.resolution,
                    level.resolution,
                ));
            };
            j = j + 1.;
        }
        i = i + 1.;
    }
}

pub fn wall(x: f32, y: f32, width: f32, height: f32) -> ComponentTree {
    (Arc::new(move |e: &mut EntityCommands| {
        e.insert((
            ShapeBundle {
                path: lyon::GeometryBuilder::build_as(&lyon::shapes::Rectangle {
                    extents: Vec2::new(width, height),
                    origin: lyon::shapes::RectangleOrigin::Center,
                }),
                ..default()
            },
            Transform2d::from_xy(x, y),
            Fill::color(Color::TEAL),
            RigidBody::Fixed,
            Collider::cuboid(width / 2., height / 2.),
            Name::new("Wall"),
        ));
    }) as EntityCommandSet)
        .into()
}
