use bevy::prelude::*;
use bevy_prototype_lyon::prelude as lyon;
use bevy_rapier2d::prelude::*;

#[derive(Clone)]
pub enum Trackable {
    TrackedEntity(Entity),
    SelfVelocity,
}

#[derive(Component)]
pub struct Tracking(pub Trackable);

#[derive(Component)]
pub struct BodyPart {
    pub parent: Entity,
    pub offset: Vec2,
}

#[derive(Component)]
pub struct Legs {
    pub animation_stage: isize,
}

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(facing_update_system)
            .add_system(move_body_parts);
    }
}

fn facing_update_system(
    todo_entities: Query<Entity, With<Tracking>>,
    mut transforms: Query<(&mut Transform, Option<&Tracking>, Option<&Velocity>)>,
) {
    // let cursor_transform = *transforms.get(cursor.0).unwrap();
    for entity in todo_entities.iter() {
        let entity = entity.clone();
        let tracking: Trackable = transforms.get(entity).unwrap().1.unwrap().0.clone();
        let target = Vec3::normalize(match tracking {
            Trackable::TrackedEntity(tracked_entity) => {
                transforms.get(entity).unwrap().0.translation
                    - transforms.get(tracked_entity).unwrap().0.translation
            }
            Trackable::SelfVelocity => transforms.get(entity).unwrap().2.unwrap().linvel.extend(0.),
        });
        let transform: &mut Transform = &mut transforms.get_mut(entity).unwrap().0;
        let right = Vec3::from((0., 0., 1.)).cross(target).normalize();
        let up = target.cross(right);
        let result = Quat::from_mat3(&Mat3::from_cols(target, right, up));
        if !result.is_nan() {
            transform.rotation = result;
        }
    }
}

fn move_body_parts(parts: Query<(Entity, &BodyPart)>, mut transforms: Query<&mut Transform>) {
    for (entity, part) in parts.iter() {
        let entity = entity.clone();
        let parent = part.parent;
        let parent_transform = transforms.get(parent).unwrap().clone();
        let mut part_transform = transforms.get_mut(entity).unwrap();
        *part_transform = parent_transform; // TODO: Calculate offsets
    }
}
