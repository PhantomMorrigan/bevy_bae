use alloc::slice;
use bevy_ecs::relationship::{RelatedSpawner, RelatedSpawnerCommands};
use core::iter::Copied;

use crate::prelude::*;

#[derive(Component, Deref, Reflect, Debug, PartialEq, Eq, Clone)]
#[relationship(relationship_target = Conditions)]
#[reflect(Component)]
pub struct ConditionOf(pub Entity);

#[derive(Component, Clone, Deref, Reflect, Debug, Default, PartialEq, Eq)]
#[relationship_target(relationship = ConditionOf, linked_spawn)]
#[reflect(Component)]
pub struct Conditions(Vec<Entity>);

impl<'a> IntoIterator for &'a Conditions {
    type Item = Entity;
    type IntoIter = Copied<slice::Iter<'a, Entity>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub type ConditionSpawner<'w> = RelatedSpawner<'w, ConditionOf>;

pub type ConditionSpawnerCommands<'w> = RelatedSpawnerCommands<'w, ConditionOf>;

#[macro_export]
macro_rules! conditions {
    [$($condition:expr),*$(,)?] => {
        ::bevy::prelude::related!($crate::prelude::Conditions[$($condition),*])
    };
}

pub use conditions;
