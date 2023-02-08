use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_composable::{ComponentTree, EntityCommandSet};
use bevy_prototype_lyon::prelude as lyon;
use bevy_prototype_lyon::prelude::ShapePlugin;
use std::sync::Arc;
use twin_stick::bevy_rapier2d::prelude::{Collider, RigidBody};

#[derive(Resource)]
pub struct Level {
    pub arena_map: Vec<Vec<u8>>,
}

fn spawn_arena_from_map(mut commands: Commands, map: Vec<Vec<u8>>) {}

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
                Transform::default(),
            ),
            RigidBody::Fixed,
            Collider::cuboid(width / 2., height / 2.),
            Transform::from_xyz(x, y, 0.0),
        ));
    }) as EntityCommandSet)
        .into()
}
