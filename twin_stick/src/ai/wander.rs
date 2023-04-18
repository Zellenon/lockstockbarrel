use std::{f32::consts::PI, iter::repeat_with};

use bevy::{
    prelude::{Component, Query, Res, ResMut, Vec2},
    reflect::Reflect,
    time::Time,
};
use bevy_turborand::{DelegatedRng, GlobalRng};

use crate::actors::Actor;

#[derive(Component, Debug, Reflect)]
pub struct PerlinWanderAI {
    pub angle_delta: f32,
    pub strength_delta: f32,
    pub angle_speed: f32,
    pub strength_speed: f32,
    pub min_strength: f32,
    pub max_strength: f32,
    pub angle: f32,
    pub current_strength: f32,
}

impl PerlinWanderAI {
    pub fn new(
        angle_speed: f32,
        strength_speed: f32,
        min_strength: f32,
        max_strength: f32,
    ) -> Self {
        Self {
            angle: 0.0,
            current_strength: 0.0,
            angle_speed,
            strength_speed,
            min_strength,
            max_strength,
            angle_delta: 0.,
            strength_delta: 0.,
        }
    }
}

impl Default for PerlinWanderAI {
    fn default() -> Self {
        PerlinWanderAI {
            angle_delta: 0.,
            strength_delta: 0.,
            angle_speed: 1.,
            strength_speed: 1.,
            min_strength: 0.,
            max_strength: 1.,
            angle: 0.,
            current_strength: 0.,
        }
    }
}

pub(crate) fn ai_wander(
    mut actors: Query<(&mut Actor, &mut PerlinWanderAI)>,
    mut rand: ResMut<GlobalRng>,
    time: Res<Time>,
) {
    let temp = repeat_with(|| rand.f32_normalized())
        .take(actors.iter().count() * 2)
        .collect::<Vec<f32>>();
    let mut nums = temp.iter();
    for (mut actor, mut wander) in actors.iter_mut() {
        let val1 = nums.next().unwrap();
        let val2 = nums.next().unwrap();

        wander.angle_delta += val1 * wander.angle_speed * time.delta().as_secs_f32();
        if wander.angle_delta > 2. * PI {
            wander.angle_delta -= 2. * PI;
        } else if wander.angle_delta < 0. {
            wander.angle_delta += 2. * PI;
        }
        wander.angle += wander.angle_delta;
        if wander.angle > 2. * PI {
            wander.angle -= 2. * PI;
        } else if wander.angle < 0. {
            wander.angle += 2. * PI;
        }

        wander.strength_delta += val2 * wander.strength_speed * time.delta().as_secs_f32();
        if wander.strength_delta > wander.max_strength {
            wander.strength_delta -= wander.max_strength - wander.min_strength;
        } else if wander.strength_delta < wander.min_strength {
            wander.strength_delta += f32::max(
                wander.min_strength,
                wander.strength_delta - wander.min_strength,
            );
        }
        wander.current_strength += wander.strength_delta;
        if wander.current_strength > wander.max_strength {
            wander.current_strength -= wander.max_strength - wander.min_strength;
        } else if wander.current_strength < wander.min_strength {
            wander.current_strength += f32::max(
                wander.min_strength,
                wander.min_strength - wander.strength_delta,
            );
        }

        actor.desired_direction += Vec2::new(
            wander.angle.cos() * wander.current_strength,
            wander.angle.sin() * wander.current_strength,
        );
    }
}
