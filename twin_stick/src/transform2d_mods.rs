use bevy::{
    prelude::{Bundle, ComputedVisibility, GlobalTransform, Handle, Image, Transform, Visibility},
    render::texture::DEFAULT_IMAGE_HANDLE,
    sprite::{Sprite, TextureAtlas, TextureAtlasSprite},
};
use bevy_mod_transform2d::transform2d::Transform2d;

#[derive(Bundle, Clone)]
pub struct Sprite2dBundle {
    pub sprite: Sprite,
    pub transform: Transform2d,
    pub _transform: Transform,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

impl Default for Sprite2dBundle {
    fn default() -> Self {
        Self {
            sprite: Default::default(),
            transform: Default::default(),
            _transform: Default::default(),
            global_transform: Default::default(),
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        }
    }
}

/// A Bundle of components for drawing a single sprite from a sprite sheet (also referred
/// to as a `TextureAtlas`)
#[derive(Bundle, Clone, Default)]
pub struct SpriteSheet2dBundle {
    /// The specific sprite from the texture atlas to be drawn, defaulting to the sprite at index 0.
    pub sprite: TextureAtlasSprite,
    /// A handle to the texture atlas that holds the sprite images
    pub texture_atlas: Handle<TextureAtlas>,
    /// Data pertaining to how the sprite is drawn on the screen
    pub _transform: Transform,
    pub transform: Transform2d,
    pub global_transform: GlobalTransform,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
}
