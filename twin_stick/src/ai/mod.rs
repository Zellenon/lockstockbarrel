use bevy::prelude::{App, IntoSystemConfig, Plugin, Query};

use crate::{
    actors::{actor_movement, Actor},
    player::player_exists,
};

use self::{
    keyboard::keyboard_input_handler,
    tracking::do_tracker_ai,
    wander::{ai_wander, PerlinWanderAI},
};

pub mod keyboard;
pub mod tracking;
pub mod wander;

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

fn normalize_ai(mut actors: Query<&mut Actor>) {
    for mut actor in actors.iter_mut() {
        actor.desired_direction = actor.desired_direction.clamp_length_max(1.);
    }
}
