use bevy::ecs::{bundle::Bundle, system::EntityCommands};
use std::sync::Arc;

use crate::content::enemies::stumbler;

//pub trait SpawnFn<T> = (Fn() -> T) + Send + Sync;
pub type StoredCommand = Arc<dyn (Fn(&mut EntityCommands)) + Send + Sync>;

pub fn store<T, U>(f: T) -> StoredCommand
where
    U: Bundle,
    T: 'static + Sync + Send + (Fn() -> U),
{
    Arc::new(move |commands: &mut EntityCommands| {
        commands.insert(f());
    })
}

pub struct SpawnCommand(Vec<StoredCommand>);

impl SpawnCommand {
    pub fn run(&self, commands: &mut EntityCommands) {
        self.0.iter().fold(commands, |e, f| {
            f(e);
            e
        });
    }
}
