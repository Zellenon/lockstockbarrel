use bevy::prelude::*;

pub trait StatHandling {
    fn register_type<T>(&mut self) -> ();
}

impl StatHandling for Commands<'_, '_> {
    fn register_type<T>(&mut self) -> () {
        todo!()
    }
}
