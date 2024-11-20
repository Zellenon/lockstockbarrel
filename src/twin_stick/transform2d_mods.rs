use bevy::{
    prelude::{
        Bundle, GlobalTransform, Handle, Image, InheritedVisibility, Reflect, Transform, Visibility,
    },
    sprite::{Sprite, TextureAtlas},
};
use bevy_mod_transform2d::transform2d::Transform2d;

#[derive(Clone, Reflect, Debug, Bundle)]
pub struct Sprite2dBundle {
    pub sprite: Sprite,
    pub transform: Transform2d,
    pub _transform: Transform,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    pub visibility: Visibility,
    pub computed_visibility: InheritedVisibility,
}

impl Default for Sprite2dBundle {
    fn default() -> Self {
        Self {
            sprite: Default::default(),
            transform: Default::default(),
            _transform: Default::default(),
            global_transform: Default::default(),
            texture: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        }
    }
}

/// A Bundle of components for drawing a single sprite from a sprite sheet (also referred
/// to as a `TextureAtlas`)
#[derive(Clone, Reflect, Debug, Bundle, Default)]
pub struct SpriteSheet2dBundle {
    /// The specific sprite from the texture atlas to be drawn, defaulting to the sprite at index 0.
    pub sprite: Sprite,
    pub texture: Handle<Image>,
    /// A handle to the texture atlas that holds the sprite images
    pub atlas: TextureAtlas,
    /// Data pertaining to how the sprite is drawn on the screen
    pub _transform: Transform,
    pub transform: Transform2d,
    pub global_transform: GlobalTransform,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: InheritedVisibility,
}
