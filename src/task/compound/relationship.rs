use crate::prelude::*;
use alloc::slice;
use bevy_ecs::relationship::{RelatedSpawner, RelatedSpawnerCommands};
use core::{fmt::Debug, iter::Copied, marker::PhantomData};

#[derive(Component, Deref, Reflect, Debug, PartialEq, Eq, Clone)]
#[relationship(relationship_target = Tasks<T>)]
#[reflect(Component)]
pub struct TaskOf<T: CompoundTask> {
    #[deref]
    #[relationship]
    entity: Entity,
    #[reflect(ignore)]
    marker: PhantomData<T>,
}

#[derive(Component, Deref, Reflect, Debug, Default, PartialEq, Eq)]
#[relationship_target(relationship = TaskOf<T>, linked_spawn)]
#[reflect(Component)]
pub struct Tasks<T: CompoundTask> {
    #[deref]
    #[relationship]
    entities: Vec<Entity>,
    #[reflect(ignore)]
    marker: PhantomData<T>,
}

impl<'a, T: CompoundTask> IntoIterator for &'a Tasks<T> {
    type Item = Entity;
    type IntoIter = Copied<slice::Iter<'a, Entity>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub type TaskSpawner<'w, T> = RelatedSpawner<'w, TaskOf<T>>;
pub type TaskSpawnerCommands<'w, T> = RelatedSpawnerCommands<'w, TaskOf<T>>;

#[macro_export]
macro_rules! tasks {
    ($compound:ty[$($condition:expr),+]) => {
        $crate::prelude::tasks!($compound[$($condition,)*])
    };
    // I know this is the same as just doing `$(,)?`, but looks like using two arms is kinder to r-a
    ($compound:ty[$($condition:expr,)*]) => {
        ::bevy::prelude::related!($crate::prelude::Tasks<$compound>[$($condition),*])
    };
}

pub use tasks;
