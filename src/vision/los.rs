use bevy::{
    ecs::{entity::Entity, system::Query},
    transform::components::Transform,
};

use super::{VisionObjects, LOS};

pub fn update_los(
    mut seers: Query<(Entity, &mut LOS)>,
    positions: Query<(Entity, &Transform), VisionObjects>,
) {
    for (e1, mut seer) in seers.iter_mut() {
        let t1 = positions.get(e1).unwrap().1.translation;
        seer.0 = positions
            .iter()
            .map(|(e, transform)| (e, transform.translation))
            .filter(|(_e2, t2)| t1.distance(*t2) < 300.)
            .map(|(e2, _t2)| e2)
            .collect()
    }
}
