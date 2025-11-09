use alloc::slice;
use bevy_ecs::relationship::{RelatedSpawner, RelatedSpawnerCommands};
use core::iter::Copied;

use crate::prelude::*;

#[derive(Component, Deref, Reflect, Debug, PartialEq, Eq, Clone)]
#[relationship(relationship_target = Effects)]
#[reflect(Component)]
pub struct EffectOf(pub Entity);

#[derive(Component, Clone, Deref, Reflect, Debug, Default, PartialEq, Eq)]
#[relationship_target(relationship = EffectOf, linked_spawn)]
#[reflect(Component)]
pub struct Effects(Vec<Entity>);

impl<'a> IntoIterator for &'a Effects {
    type Item = Entity;
    type IntoIter = Copied<slice::Iter<'a, Entity>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub type EffectSpawner<'w> = RelatedSpawner<'w, EffectOf>;

pub type EffectSpawnerCommands<'w> = RelatedSpawnerCommands<'w, EffectOf>;

#[macro_export]
macro_rules! effects {
    [$($effect:expr),*$(,)?] => {
        ::bevy::prelude::related!($crate::prelude::Effects[$($effect),*])
    };
}

pub use effects;
