use std::ops;
use std::sync::Arc;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

pub type EntityCommandSet = Arc<dyn Fn(&mut EntityCommands) + Send + Sync>;

#[derive(Clone)]
pub struct ComponentTree {
    pub commands: Vec<EntityCommandSet>,
    pub children: Vec<ComponentTree>,
}

impl From<EntityCommandSet> for ComponentTree {
    fn from(value: EntityCommandSet) -> Self {
        ComponentTree {
            commands: vec![value],
            children: Vec::new(),
        }
    }
}

impl ops::Add<ComponentTree> for ComponentTree {
    type Output = ComponentTree;

    fn add(self, rhs: ComponentTree) -> Self::Output {
        Self {
            commands: [self.commands.as_slice(), rhs.commands.as_slice()].concat(),
            children: [self.children.as_slice(), rhs.children.as_slice()].concat(),
        }
    }
}

impl ops::Add<EntityCommandSet> for ComponentTree {
    type Output = ComponentTree;

    fn add(self, rhs: EntityCommandSet) -> Self::Output {
        Self {
            commands: [self.commands.as_slice(), vec![rhs].as_slice()].concat(),
            children: self.children,
        }
    }
}

impl ops::Shl<ComponentTree> for ComponentTree {
    type Output = ComponentTree;

    fn shl(self, rhs: ComponentTree) -> Self::Output {
        Self {
            commands: self.commands,
            children: [self.children.as_slice(), vec![rhs].as_slice()].concat(),
        }
    }
}

impl Default for ComponentTree {
    fn default() -> Self {
        ComponentTree {
            commands: Vec::default(),
            children: Vec::default(),
        }
    }
}

pub fn spawn_complex(commands: &mut Commands, component_tree: ComponentTree) {
    let entity = &mut commands.spawn_empty();
    spawn_complex_inner(entity, &component_tree);
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
