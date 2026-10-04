use bevy::ecs::{bundle::Bundle, system::EntityCommands};
use std::sync::Arc;

//pub trait SpawnFn<T> = (Fn() -> T) + Send + Sync;
pub type StoredCommand = Arc<dyn (Fn(&mut EntityCommands)) + Send + Sync>;

pub fn store<T, U>(f: T) -> StoredCommand
where
    U: Bundle,
    T: 'static + Send + Sync + (Fn() -> U),
{
    Arc::new(move |commands: &mut EntityCommands| {
        commands.insert(f());
    })
}

pub struct SpawnCommand(pub Vec<StoredCommand>);

impl SpawnCommand {
    pub fn run(&self, commands: &mut EntityCommands) {
        self.0.iter().fold(commands, |e, f| {
            f(e);
            e
        });
    }
}

impl From<Vec<StoredCommand>> for SpawnCommand {
    fn from(value: Vec<StoredCommand>) -> Self {
        Self(value)
    }
}

impl From<StoredCommand> for SpawnCommand {
    fn from(value: StoredCommand) -> Self {
        Self(vec![value])
    }
}
