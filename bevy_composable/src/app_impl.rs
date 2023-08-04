use std::sync::Arc;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::tree::{ComponentTree, EntityCommandSet};

pub trait ComplexSpawnable {
    fn spawn_complex(&mut self, tree: ComponentTree) -> Entity;
}

impl ComplexSpawnable for Commands<'_, '_> {
    fn spawn_complex(&mut self, tree: ComponentTree) -> Entity {
        let entity = &mut self.spawn_empty();
        spawn_complex_inner(entity, &tree);
        entity.id()
    }
}

fn spawn_complex_inner(entity: &mut EntityCommands, component_tree: &ComponentTree) {
    for command in component_tree.commands.iter() {
        command(entity);
    }
    for child in component_tree.children.iter() {
        entity.with_children(|w| {
            let mut child_entity = w.spawn_empty();
            spawn_complex_inner(&mut child_entity, child);
        });
    }
}

// impl<T> From<dyn Component<Storage = T>> for ComponentTree {
//     fn from(value: dyn Component<Storage = T>) -> Self {
//         let func = move |parent: &mut EntityCommands| {
//             parent.insert(value);
//         };
//         (Arc::new(func) as EntityCommandSet).into()
//     }
// }

pub fn from<T>(value: impl Component<Storage = T> + Clone) -> EntityCommandSet {
    let func = move |parent: &mut EntityCommands| {
        parent.insert(value.clone());
    };
    (Arc::new(func) as EntityCommandSet).into()
}

pub trait ComponentTreeable {
    fn tree(self) -> ComponentTree;
}

impl<W, T> ComponentTreeable for W
where
    W: Component<Storage = T> + Clone,
{
    fn tree(self) -> ComponentTree {
        let func = move |parent: &mut EntityCommands| {
            parent.insert(self.clone());
        };
        (Arc::new(func) as EntityCommandSet).into()
    }
}
