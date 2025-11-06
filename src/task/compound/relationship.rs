use crate::prelude::*;
use crate::task::BaeTask;
use alloc::slice;
use bevy_ecs::relationship::{RelatedSpawner, RelatedSpawnerCommands};
use core::{fmt::Debug, iter::Copied};

#[derive(Component, Deref, Reflect, Debug, PartialEq, Eq, Clone)]
#[relationship(relationship_target = BaeTasks)]
#[reflect(Component)]
pub struct BaeTaskOf(pub Entity);

#[derive(Component, Deref, Reflect, Debug, Default, PartialEq, Eq)]
#[relationship_target(relationship = BaeTaskOf, linked_spawn)]
#[reflect(Component)]
pub struct BaeTasks(Vec<Entity>);

impl<'a> IntoIterator for &'a BaeTasks {
    type Item = Entity;
    type IntoIter = Copied<slice::Iter<'a, Entity>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub type BaeTaskSpawner<'w> = RelatedSpawner<'w, BaeTaskOf>;
pub type BaeTaskSpawnerCommands<'w> = RelatedSpawnerCommands<'w, BaeTaskOf>;

#[macro_export]
macro_rules! tasks {
    [$($condition:expr),*$(,)?] => {
        ::bevy::prelude::related!($crate::prelude::BaeTasks[$($crate::prelude::IntoTaskBundle::into_task_bundle($condition)),*])
    };
}

pub use tasks;

#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a valid task bundle. The first element must be either an `Operator` or a component that implementes `CompositeTask`, like `Select` or `Sequence`.",
    label = "invalid task bundle"
)]
pub trait IntoTaskBundle {
    fn into_task_bundle(self) -> impl Bundle;
}

impl<B: BaeTask> IntoTaskBundle for B {
    fn into_task_bundle(self) -> impl Bundle {
        self
    }
}

macro_rules! impl_into_task_bundle {
    ($($C:ident),*) => {
        impl<B: BaeTask, $($C: Bundle,)*> IntoTaskBundle for (B, $($C),*) {
            #[allow(non_snake_case, reason = "tuple unpack")]
            fn into_task_bundle(self) -> impl Bundle {
                let (b, $($C),* ) = self;
                (b, $($C),*)
            }
        }
    }
}

variadics_please::all_tuples!(impl_into_task_bundle, 0, 14, C);
