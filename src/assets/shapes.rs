use bevy::{
    asset::{Assets, Handle},
    ecs::system::{ResMut, Resource},
    math::primitives::{Circle, Rectangle},
    reflect::Reflect,
    render::mesh::Mesh,
};

#[derive(Resource, Reflect, Clone, Debug, PartialEq)]
pub struct ShapeResources {
    pub rectangle: Handle<Mesh>,
    pub circle: Handle<Mesh>,
}

impl ShapeResources {
    pub fn init(mut meshes: ResMut<Assets<Mesh>>) {
        ShapeResources {
            rectangle: Rectangle::new(1.0, 1.0),
            circle: Circle::new(0.5),
        }
    }
}
