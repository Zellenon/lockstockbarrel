use bevy::prelude::*;
use bevy_composable::ComponentTree;
use iyes_loopless::prelude::AppLooplessStateExt;
use twin_stick::actors::ActorBundle;

use crate::{
    content::{enemies::basic_walker, shift_pos},
    states::AppState,
};

pub struct WavePlugin;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnWaveEvent>();
        app.add_enter_system(AppState::Game, setup);
    }
}

pub struct SpawnWaveEvent;

pub struct SpawnWave(Vec<ComponentTree>, SpawnCondition);

pub struct SpawnCommand {}

pub enum SpawnCondition {
    OnTimer(f32),
    DeathPercentage(f32),
    And(Box<SpawnCondition>),
    Or(Box<SpawnCondition>),
    DeathOfTaggedEntity(Entity),
    DeathOfTaggedEntities(Box<[Entity]>),
}

#[derive(Component)]
pub struct WaveManager {
    waves: Vec<SpawnWave>,
}

fn setup(mut commands: Commands) {
    commands.spawn(WaveManager {
        waves: vec![SpawnWave(
            vec![
                basic_walker() + shift_pos((500., 0.)),
                basic_walker() + shift_pos((0., 500.)),
                basic_walker() + shift_pos((-500., 0.)),
            ],
            SpawnCondition::OnTimer(4.),
        )],
    });
}
