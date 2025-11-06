use bevy_ecs::world::DeferredWorld;

use crate::prelude::*;

pub mod builtin;
pub mod relationship;

pub trait Condition: Component {
    fn is_satisfied(&mut self, world: EntityWorldMut) -> bool;
}
