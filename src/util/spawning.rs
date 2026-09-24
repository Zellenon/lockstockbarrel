use std::sync::Arc;

use bevy::ecs::{bundle::Bundle, system::EntityCommands};

pub trait Spawnable = Bundle + Clone;
pub type StoredCommand = Arc<dyn Fn(&mut EntityCommands) + Send + Sync>;

pub fn store<T>(bundle: T) -> StoredCommand
where
    T: Bundle + Clone,
{
    Arc::new(move |commands: &mut EntityCommands| {
        commands.insert(bundle.clone());
    })
}
