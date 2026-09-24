use bevy::{
    app::App,
    ecs::{
        bundle::Bundle,
        entity::Entity,
        hierarchy::ChildOf,
        observer::On,
        query::{Or, With},
    },
    prelude::{Commands, Component, Query, Transform},
    scene::SceneComponent,
    transform::components::GlobalTransform,
};

use crate::{
    action_system::actuator::Actuate,
    twin_stick::{actors::Actor, weapons::Weapon},
    util::{
        add_observer_to_component,
        spawning::{store, StoredCommand},
    },
};

#[derive(Component, Clone)]
pub struct SpawnAction(pub Vec<StoredCommand>);

#[derive(Component, Clone, Debug)]
#[relationship_target(relationship = SpawnedBy)]
pub struct Spawned(Vec<Entity>);

#[derive(Component, Clone, PartialEq, Hash, Debug)]
#[relationship(relationship_target = Spawned)]
pub struct SpawnedBy(pub Entity);

impl SpawnAction {
    pub fn setup(app: &mut App) {
        // app.register_type::<SpawnAction>();
        app.add_observer(add_observer_to_component::<SpawnAction, _, _, _, _>(
            do_spawn_action,
        ));
    }
}

pub fn spawn(bundle: impl Bundle + Clone) -> SpawnAction {
    SpawnAction(vec![store(bundle)])
}

pub fn spawns<T: Iterator<Item = impl Bundle + Clone>>(bundles: T) -> SpawnAction {
    SpawnAction(bundles.map(|w| store(w)).collect())
}

pub fn do_spawn_action(
    trigger: On<Actuate>,
    spawners: Query<(Entity, &SpawnAction, &GlobalTransform)>,
    attackers: Query<Entity, Or<(With<Actor>, With<Weapon>)>>,
    parents: Query<&ChildOf>,
    mut commands: Commands,
) {
    if let Ok((e, spawn_action, transform)) = spawners.get(trigger.event().0) {
        let (scale, rotation, translation) = transform.to_scale_rotation_translation();
        let spawned_transform = Transform {
            translation,
            rotation,
            scale,
        };
        for payload in spawn_action.0.iter() {
            if let Some(attacker) = std::iter::once(e)
                .chain(parents.iter_ancestors(e))
                .filter(|w| attackers.get(*w).is_ok())
                .next()
            // If there's a first ancestor with Weapon/Actor
            {
                payload(&mut commands.spawn((spawned_transform, SpawnedBy(attacker))));
            } else {
                payload(&mut commands.spawn((spawned_transform, SpawnedBy(e))));
            }
        }
    }
}
