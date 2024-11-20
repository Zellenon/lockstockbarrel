use bevy::prelude::Reflect;
use std::marker::PhantomData;

use bevy::{
    prelude::{
        in_state, App, Commands, Component, Entity, Event, EventReader, IntoSystemConfigs, Parent,
        Plugin, Query, Res, Update, With,
    },
    time::{Time, Timer, TimerMode},
};
use bevy_mod_transform2d::transform2d::Transform2d;

use crate::{meta_states::PluginControlState, player::CursorTracker};

#[derive(Clone, Copy, PartialEq, Eq, Reflect, Debug)]
pub struct WeaponPlugin<T: PluginControlState> {
    _z: PhantomData<T>,
}

impl<T: PluginControlState> Default for WeaponPlugin<T> {
    fn default() -> Self {
        Self { _z: PhantomData }
    }
}

impl<T: PluginControlState> Plugin for WeaponPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_event::<FireWeaponEvent>();
        app.add_systems(
            Update,
            (
                fire_weapons,
                tick_cooldowns,
                reset_weapon_cooldowns,
                enable_weapons_on_cooldown,
            )
                .run_if(in_state(T::active_state())),
        );
    }
}

#[derive(Component)]
pub struct Weapon {
    pub can_fire: bool,
    pub fire_func: Box<dyn Fn(&mut WeaponArguments) + Send + Sync>,
    pub fire_mode: WeaponFireMode,
}

#[derive(Clone, Copy, PartialEq, Eq, Reflect, Debug)]
pub enum WeaponFireMode {
    SemiAuto,
    FullAuto,
}

pub struct WeaponArguments<'c, 'w, 's, 'c2, 'w2, 's2> {
    pub commands: &'c mut Commands<'w, 's>,
    pub cursor: Entity,
    pub target: Option<Entity>,
    pub parent: Entity,
    pub transforms: Query<'c2, 'w2, &'s2 Transform2d>,
}

#[derive(Clone, PartialEq, Reflect, Debug, Component)]
pub struct Cooldown {
    pub max: f32,
    pub timer: Timer,
}

impl Cooldown {
    pub fn new(max: f32) -> Self {
        Self {
            max,
            timer: Timer::from_seconds(max, TimerMode::Once),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Reflect, Debug, Event)]
pub struct FireWeaponEvent {
    pub weapon: Entity,
    pub target: Option<Entity>,
}

pub fn fire_weapons(
    mut events: EventReader<FireWeaponEvent>,
    mut commands: Commands,
    cursor: Query<Entity, With<CursorTracker>>,
    weapons: Query<(&Weapon, &Parent)>,
    transforms: Query<&Transform2d>,
) {
    let mut args = WeaponArguments {
        commands: &mut commands,
        cursor: cursor.single(),
        target: None,
        parent: Entity::from_raw(0),
        transforms,
    };
    for FireWeaponEvent { weapon, target } in events.read() {
        args.target = *target;
        args.parent = weapons.get(*weapon).unwrap().1.get();
        (*weapons.get(*weapon).unwrap().0.fire_func)(&mut args);
    }
}

fn reset_weapon_cooldowns(
    mut events: EventReader<FireWeaponEvent>,
    mut weapon_query: Query<(&mut Weapon, &mut Cooldown)>,
) {
    for FireWeaponEvent {
        weapon: weapon_entity,
        target: _,
    } in events.read()
    {
        let (mut weapon, mut cooldown) = weapon_query.get_mut(*weapon_entity).unwrap();
        weapon.can_fire = false;
        cooldown.timer = Timer::from_seconds(cooldown.max, TimerMode::Once);
    }
}

fn enable_weapons_on_cooldown(mut weapon_query: Query<(&mut Weapon, &mut Cooldown)>) {
    for (mut weapon, _) in weapon_query
        .iter_mut()
        .filter(|(w, c)| (!w.can_fire) && c.timer.finished())
    {
        weapon.can_fire = true;
    }
}

fn tick_cooldowns(mut cooldown_query: Query<&mut Cooldown>, time: Res<Time>) {
    for mut cooldown in cooldown_query.iter_mut() {
        cooldown.timer.tick(time.delta());
    }
}
