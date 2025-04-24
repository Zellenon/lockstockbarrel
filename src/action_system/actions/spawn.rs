use bevy::{
    app::App,
    ecs::{
        entity::Entity,
        query::{Or, With},
    },
    hierarchy::{HierarchyQueryExt, Parent},
    prelude::{Commands, Component, Query, Transform, Trigger},
    transform::components::GlobalTransform,
};
use bevy_composable::{
    app_impl::{ComplexSpawnable, ComponentTreeable},
    tree::ComponentTree,
};

use crate::{
    action_system::actuator::Actuate,
    twin_stick::{actors::Actor, weapons::Weapon},
    util::add_observer_to_component,
};

#[derive(Component, Clone)]
pub struct SpawnAction {
    pub payload: Vec<ComponentTree>,
}

#[derive(Component, Clone, PartialEq, Hash, Debug)]
pub struct SpawnedBy(pub Entity);

impl SpawnAction {
    pub fn spawn(tree: ComponentTree) -> Self {
        Self {
            payload: vec![tree],
        }
    }

    pub fn spawns<T: Iterator<Item = ComponentTree>>(trees: T) -> Self {
        Self {
            payload: trees.collect(),
        }
    }

    pub fn setup(app: &mut App) {
        // app.register_type::<SpawnAction>();
        app.add_observer(add_observer_to_component::<SpawnAction, _, _, _, _>(
            do_spawn_action,
        ));
    }
}

pub fn spawn(tree: ComponentTree) -> ComponentTree {
    SpawnAction::spawn(tree).store()
}

pub fn spawns<T: Iterator<Item = ComponentTree>>(trees: T) -> ComponentTree {
    SpawnAction::spawns(trees).store()
}

pub fn do_spawn_action(
    trigger: Trigger<Actuate>,
    spawners: Query<(Entity, &SpawnAction, &GlobalTransform)>,
    attackers: Query<Entity, Or<(With<Actor>, With<Weapon>)>>,
    parents: Query<&Parent>,
    mut commands: Commands,
) {
    if let Ok((e, spawn_action, transform)) = spawners.get(trigger.entity()) {
        for payload in spawn_action.payload.iter() {
            let (scale, rotation, translation) = transform.to_scale_rotation_translation();
            let spawned_transform = Transform {
                translation,
                rotation,
                scale,
            };

            if let Some(attacker) = std::iter::once(e)
                .chain(parents.iter_ancestors(e))
                .filter(|w| attackers.get(*w).is_ok())
                .next()
            // If there's a first ancestor with Weapon/Actor
            {
                commands
                    .compose(payload.clone() + (spawned_transform, SpawnedBy(attacker)).store());
            } else {
                commands.compose(payload.clone() + spawned_transform.store());
            }
        }
    }
}
