use bevy::prelude::{Component, Plugin, Query, Transform, Update, Vec2};
use bevy_vector_shapes::{
    prelude::ShapePainter,
    shapes::{DiscPainter, RectPainter},
    Shape2dPlugin,
};

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, (draw_circles, draw_rects));

        app.add_plugins(Shape2dPlugin::default());
    }
}

#[derive(Component)]
pub struct Circle {
    pub size: f32,
    pub color: bevy::render::color::Color,
}

#[derive(Component)]
pub struct Square {
    pub size: f32,
    pub color: bevy::render::color::Color,
}

impl Circle {
    pub fn new(size: f32, color: bevy::render::color::Color) -> Self {
        Self { size, color }
    }
}

impl Square {
    pub fn new(size: f32, color: bevy::render::color::Color) -> Self {
        Self { size, color }
    }
}

pub fn draw_rects(rects: Query<(&Transform, &Square)>, mut painter: ShapePainter) {
    for (transform, rect) in rects.iter() {
        painter.translate(transform.translation);
        painter.hollow = false;
        painter.color = rect.color;
        painter.rect(Vec2::new(rect.size, rect.size));
    }
}

pub fn draw_circles(circles: Query<(&Transform, &Circle)>, mut painter: ShapePainter) {
    for (transform, circle) in circles.iter() {
        painter.translate(transform.translation);
        painter.hollow = false;
        painter.color = circle.color;
        painter.circle(circle.size);
    }
}
