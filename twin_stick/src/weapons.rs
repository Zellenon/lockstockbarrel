use bevy::{
    prelude::{App, Commands, Component, Entity, EventReader, Parent, Plugin, Query, Res, With},
    time::{Time, Timer, TimerMode},
};
use bevy_mod_transform2d::transform2d::Transform2d;

use crate::player::CursorTracker;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FireWeaponEvent>()
            .add_system(fire_weapons)
            .add_system(tick_cooldowns)
            .add_system(reset_weapon_cooldowns)
            .add_system(enable_weapons_on_cooldown);
    }
}

#[derive(Component)]
pub struct Weapon {
    pub can_fire: bool,
    pub fire_mode: WeaponFireMode,
    pub fire_func: Box<dyn Fn(&mut WeaponArguments) + Send + Sync>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
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

#[derive(Component)]
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
    for FireWeaponEvent { weapon, target } in events.iter() {
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
    } in events.iter()
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
