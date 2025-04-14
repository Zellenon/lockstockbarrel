use bevy::input::mouse::MouseButton;
use bevy::prelude::Reflect;
use bevy::prelude::{Component, KeyCode, Query, With};
use leafwing_input_manager::{prelude::{ActionState, InputMap, VirtualDPad}, Actionlike, InputControlKind, InputManagerBundle};
use strum_macros::EnumIter;

use super::super::actors::Actor;

#[derive(Component, Clone, Copy, PartialEq, Eq, Reflect, Debug)]
pub struct KeyboardAI;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Reflect, Default, EnumIter)]
pub(crate) enum PlayerAction {
    #[default]
    Walk,
    Shoot1,
    Shoot2,
    Shoot3,
    Shoot4,
}

impl Actionlike for PlayerAction {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            PlayerAction::Walk => InputControlKind::DualAxis,
            _ => InputControlKind::Button,
        }
    }
}

pub(crate) fn create_player_action_input_manager_bundle() -> InputManagerBundle<PlayerAction> {
    InputManagerBundle::with_map(
        InputMap::new([
            (PlayerAction::Shoot3, KeyCode::ShiftLeft),
            (PlayerAction::Shoot4, KeyCode::KeyF),
        ])
            .with(PlayerAction::Shoot1, MouseButton::Left)
            .with(PlayerAction::Shoot2, MouseButton::Right)
            .with_dual_axis(
                PlayerAction::Walk,
                VirtualDPad::new(KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyD)
            )
    )
}

pub(crate) fn keyboard_input_handler(
    mut ais: Query<(&mut Actor, &ActionState<PlayerAction>), With<KeyboardAI>>,
) {
    for mut actor in ais.iter_mut() {
        actor.0.desired_direction = actor.1.axis_pair(&PlayerAction::Walk);
    }
}
