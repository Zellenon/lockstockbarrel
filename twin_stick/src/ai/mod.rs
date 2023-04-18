use std::{f32::consts::PI, iter::repeat_with};

use bevy::{
    prelude::{
        App, Component, Input, IntoSystemConfig, KeyCode, Plugin, Query, Res, ResMut, Vec2, With,
    },
    reflect::Reflect,
    time::Time,
};
use bevy_mod_transform2d::transform2d::Transform2d;
use bevy_turborand::prelude::*;

use crate::{
    actors::{actor_movement, Actor},
    player::{player_exists, Player},
};

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(do_tracker_ai.run_if(player_exists))
            .add_system(keyboard_input_handler.run_if(player_exists))
            .add_system(ai_wander)
            .add_system(normalize_ai.after(do_tracker_ai).after(ai_wander))
            .add_system(actor_movement.after(normalize_ai));
        app.register_type::<PerlinWanderAI>();
    }
}

#[derive(Component)]
pub struct TrackerAI {
    pub precision: f32,
}

pub(crate) fn do_tracker_ai(
    player: Query<&Transform2d, With<Player>>,
    mut ais: Query<(&mut Actor, &Transform2d, &TrackerAI)>,
) {
    let player_pos = player.single().translation;
    for (mut enemy, transform, tracker) in ais.iter_mut() {
        enemy.desired_direction +=
            tracker.precision * (player_pos - transform.translation).clamp_length_max(1.);
    }
}

#[derive(Component)]
pub struct KeyboardAI;

fn keyboard_input_handler(
    keyboard_input: Res<Input<KeyCode>>,
    mut ais: Query<&mut Actor, With<KeyboardAI>>,
) {
    let mut total_force = Vec2::new(0., 0.);
    if keyboard_input.pressed(KeyCode::A) {
        total_force.x += -1.;
    }
    if keyboard_input.pressed(KeyCode::D) {
        total_force.x += 1.;
    }
    if keyboard_input.pressed(KeyCode::W) {
        total_force.y += 1.;
    }
    if keyboard_input.pressed(KeyCode::S) {
        total_force.y += -1.;
    }
    for mut actor in ais.iter_mut() {
        actor.desired_direction = total_force;
    }
}

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

fn ai_wander(
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

fn normalize_ai(mut actors: Query<&mut Actor>) {
    for mut actor in actors.iter_mut() {
        actor.desired_direction = actor.desired_direction.clamp_length_max(1.);
    }
}
