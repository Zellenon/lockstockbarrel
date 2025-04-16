use bevy::prelude::Reflect;
use bevy::prelude::{App, IntoSystemConfigs, Plugin, Query, Update};
use keyboard::PlayerAction;
use leafwing_input_manager::plugin::InputManagerPlugin;

use super::{
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

#[derive(Clone, Copy, PartialEq, Eq, Reflect, Debug)]
pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PerlinWanderAI>()
            .register_type::<PlayerAction>();
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
        app.add_systems(
            Update,
            (
                do_tracker_ai.run_if(player_exists),
                keyboard_input_handler.run_if(player_exists),
                ai_wander,
                normalize_ai.after(do_tracker_ai).after(ai_wander),
                actor_movement.after(normalize_ai),
            ),
        );
    }
}

fn normalize_ai(mut actors: Query<&mut Actor>) {
    for mut actor in actors.iter_mut() {
        actor.desired_direction = actor.desired_direction.clamp_length_max(1.);
    }
}
