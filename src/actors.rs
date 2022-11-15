use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

use crate::utils::get_angle;

#[derive(Component)]
pub struct Actor;

#[derive(Component, Inspectable)]
pub struct Tracking(pub Option<Entity>);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Legs {
    pub animation_stage: isize,
    pub stroke: isize,
    pub max_scale: f32,
}

impl Default for Legs {
    fn default() -> Self {
        Self {
            animation_stage: 0,
            stroke: 1,
            max_scale: 1.,
        }
    }
}

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Legs>()
            .add_system(facing_update_system)
            .add_system(animate_legs);
    }
}

fn facing_update_system(
    todo_entities: Query<Entity, (With<Tracking>, With<Transform>, With<Parent>)>,
    mut transforms: Query<(
        &GlobalTransform,
        Option<&Tracking>,
        Option<&Velocity>,
        &mut Transform,
        Option<&Parent>,
    )>,
    parents: Query<&Velocity>,
) {
    for entity in todo_entities.iter() {
        let entity = entity.clone();
        let target = transforms.get(entity).unwrap().1.unwrap().0;
        let direction = match target {
            Some(target_entity) => {
                transforms.get(target_entity).unwrap().0.translation()
                    - transforms.get(entity).unwrap().0.translation()
            }
            None => match parents.get(transforms.get(entity).unwrap().4.unwrap().get()) {
                Ok(parent_vel) => (parent_vel.linvel * -1.).extend(0.),
                Err(_) => Vec3::Y,
            },
        };
        let transform: &mut Transform = &mut transforms.get_mut(entity).unwrap().3;
        let result = get_angle(direction);
        if !result.is_nan() {
            transform.rotation = result;
        }
    }
}

fn animate_legs(mut legs: Query<(&mut Transform, &mut Legs, &Parent)>, parents: Query<&Velocity>) {
    for (mut transform, mut legs, parent) in legs.iter_mut() {
        if legs.animation_stage > 199 {
            legs.stroke = -1;
        }
        if legs.animation_stage < -199 {
            legs.stroke = 1;
        }
        match parents.get(parent.get()) {
            Ok(parent_vel) => {
                let parent_speed = parent_vel.linvel.length() as isize;
                if parent_speed > 0 {
                    legs.animation_stage += parent_speed * legs.stroke / 30;
                } else {
                    legs.animation_stage = (legs.animation_stage as f32 * 0.9) as isize;
                }
                transform.scale = Vec3::new(1., legs.animation_stage as f32 / 100., 1.);
            }
            Err(e) => println!("{}", e),
        };
    }
}
