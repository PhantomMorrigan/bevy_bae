use alloc::slice;
use bevy_ecs::relationship::{RelatedSpawner, RelatedSpawnerCommands};
use core::iter::Copied;

use crate::prelude::*;

#[derive(Component, Deref, Reflect, Debug, PartialEq, Eq, Clone)]
#[relationship(relationship_target = Conditions)]
#[reflect(Component)]
pub struct ConditionOf(pub Entity);

#[derive(Component, Deref, Reflect, Debug, Default, PartialEq, Eq)]
#[relationship_target(relationship = ConditionOf, linked_spawn)]
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
        ::bevy::prelude::related!($crate::prelude::Conditions[$($crate::prelude::IntoConditionBundle::into_condition_bundle($condition)),*])
    };
}

pub use conditions;

pub trait IntoConditionBundle {
    /// Returns a bundle for a binding.
    fn into_condition_bundle(self) -> impl Bundle;
}

impl<B: IntoCondition> IntoConditionBundle for B {
    fn into_condition_bundle(self) -> impl Bundle {
        self.into_condition()
    }
}

macro_rules! impl_into_binding_bundle {
    ($($C:ident),*) => {
        impl<B: IntoCondition, $($C: Bundle,)*> IntoConditionBundle for (B, $($C),*) {
            #[allow(non_snake_case, reason = "tuple unpack")]
            fn into_condition_bundle(self) -> impl Bundle {
                let (b, $($C),* ) = self;
                (b.into_condition(), $($C),*)
            }
        }
    }
}

variadics_please::all_tuples!(impl_into_binding_bundle, 0, 14, C);
