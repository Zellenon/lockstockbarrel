use avian2d::prelude::PhysicsLayer;

#[derive(PhysicsLayer, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum GamePhysicsLayer {
    #[default]
    Default, // Layer 0 - the default layer that objects are assigned to
    Player,
    Enemy,
    MapSolid,
    MapDynamic,
    Ethereal,
    Bullet,
}
