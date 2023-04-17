use std::ops;
use std::sync::Arc;

use bevy::ecs::system::EntityCommands;

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
