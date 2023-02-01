use bevy::prelude::*;

pub struct LeveleventManagerPlugin;

impl Plugin for LeveleventManagerPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

trait LevelEvent {}

#[derive(Component)]
struct LeveleventManager<T, A, T2, A2> {
    pub activation_condition: Box<dyn System<In = T, Out = A>>,
    pub priming_function: Option<Box<dyn System<In = T2, Out = A2>>>,
    pub state: LEManagerState,
    pub fire_event: Box<dyn LevelEvent>,
    // Children
}

#[derive(Component)]
struct BringAlive;

#[derive(Component)]
struct FireLEManager;

pub enum LEManagerState {
    Off,
    Live,
    Fired,
}

fn testLEManagerSetup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let head_tex = asset_server.load("img/placeholder_head.png");
    let leg_tex = asset_server.load("img/placeholder_legs.png");
    spawn_complex(
        &mut commands,
        basic_walker(head_tex.clone(), leg_tex.clone()) + shift_pos((500., 0.)),
    );
    spawn_complex(
        &mut commands,
        basic_walker(head_tex.clone(), leg_tex.clone()) + shift_pos((0., 500.)),
    );
    spawn_complex(
        &mut commands,
        basic_walker(head_tex.clone(), leg_tex.clone()) + shift_pos((-500., 0.)),
    );
}

fn activateTimer
