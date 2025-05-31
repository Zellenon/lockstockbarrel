use bevy::{
    color::palettes::css::GREY,
    ecs::system::{Res, ResMut, Resource},
    gizmos::gizmos::Gizmos,
    math::Vec2,
    reflect::Reflect,
    time::Timer,
};

#[derive(Default, Debug, Reflect)]
pub struct Arrow {
    pub position: Vec2,
    pub direction: Vec2,
    pub duration: Timer,
}

#[derive(Reflect, Resource, Default)]
pub struct Arrows(pub Vec<Arrow>);

pub fn display_arrows(arrows: Res<Arrows>, mut gizmos: Gizmos) {
    for arrow in &arrows.0 {
        gizmos.arrow_2d(
            arrow.position,
            arrow.position + (arrow.direction.normalize() * 40.),
            GREY,
        );
    }
}
