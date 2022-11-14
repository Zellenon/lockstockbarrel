use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::{
    Collider, ColliderMassProperties, ExternalForce, RigidBody, Velocity,
};

use crate::player::CursorTracker;

#[derive(Component)]

pub struct Projectile;

pub struct WeaponArguments<'c, 'w, 's, 'C, 'W, 'S> {
    pub commands: &'c mut Commands<'w, 's>,
    pub cursor: Entity,
    pub target: Option<Entity>,
    pub parent: Entity,
    pub transforms: Query<'C, 'W, &'S Transform>,
}

#[derive(Component)]
pub struct Weapon {
    fire_func: Box<dyn Fn(&mut WeaponArguments) + Send + Sync>,
}

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FireWeaponEvent>().add_system(fire_weapons);
    }
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    projectile: Projectile,
    visibility: Visibility,
    transform: Transform,
    global_transform: GlobalTransform,
    rigidbody: RigidBody,
    velocity: Velocity,
    mass_properties: ColliderMassProperties,
    collider: Collider,
}

impl Default for ProjectileBundle {
    fn default() -> Self {
        Self {
            projectile: Projectile,
            visibility: Visibility { is_visible: true },
            velocity: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            rigidbody: RigidBody::Dynamic,
            mass_properties: ColliderMassProperties::Density(1.),
            collider: Collider::ball(5.),
        }
    }
}

pub fn Peashooter() -> Weapon {
    Weapon {
        fire_func: Box::new(move |a: &mut WeaponArguments| {
            let parent_transform = a.transforms.get(a.parent).unwrap().clone();
            let cursor_transform = a.transforms.get(a.cursor).unwrap().clone();
            let fire_direction =
                Vec3::normalize(cursor_transform.translation - parent_transform.translation);
            a.commands
                .spawn_bundle(ProjectileBundle {
                    velocity: Velocity {
                        linvel: fire_direction.truncate() * 5000.,
                        angvel: 0.,
                    },
                    ..Default::default()
                })
                .insert_bundle(GeometryBuilder::build_as(
                    &shapes::Circle {
                        radius: 5.,
                        center: Vec2::ZERO,
                    },
                    DrawMode::Outlined {
                        fill_mode: FillMode::color(Color::WHITE),
                        outline_mode: StrokeMode::color(Color::BLACK),
                    },
                    parent_transform
                        .with_translation(parent_transform.translation + fire_direction * 20.),
                ));
        }),
    }
}

pub struct FireWeaponEvent {
    pub weapon: Entity,
    pub target: Option<Entity>,
}

pub fn fire_weapons(
    mut events: EventReader<FireWeaponEvent>,
    mut commands: Commands,
    cursor: Query<Entity, With<CursorTracker>>,
    weapons: Query<(&Weapon, &Parent), With<Weapon>>,
    transforms: Query<&Transform>,
) {
    let mut args = WeaponArguments {
        commands: &mut commands,
        cursor: cursor.single(),
        target: None,
        parent: Entity::from_raw(0),
        transforms: transforms,
    };
    for FireWeaponEvent { weapon, target } in events.iter() {
        args.target = *target;
        args.parent = weapons.get(*weapon).unwrap().1.get();
        (*weapons.get(*weapon).unwrap().0.fire_func)(&mut args);
    }
    // args.target = None;
}
