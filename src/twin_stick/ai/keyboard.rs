use bevy::{
    input::mouse::MouseButton,
    prelude::{Component, KeyCode, Query, Reflect, With},
};
use leafwing_input_manager::{
    prelude::{ActionState, InputMap, VirtualDPad},
    Actionlike, InputControlKind,
};
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

pub(crate) fn create_player_action_input_manager_bundle() -> InputMap<PlayerAction> {
    let mut map = InputMap::default();
    map.insert(PlayerAction::Shoot3, KeyCode::Space);
    map.insert(PlayerAction::Shoot4, KeyCode::ShiftLeft);
    map.insert(PlayerAction::Shoot1, MouseButton::Left);
    map.insert(PlayerAction::Shoot2, MouseButton::Right);
    map.insert_dual_axis(
        PlayerAction::Walk,
        VirtualDPad::new(KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyD),
    );
    map
}

pub(crate) fn keyboard_input_handler(
    mut ais: Query<(&mut Actor, &ActionState<PlayerAction>), With<KeyboardAI>>,
) {
    for (mut actor, action_state) in ais.iter_mut() {
        actor.desired_direction = action_state.axis_pair(&PlayerAction::Walk);
    }
}
