use crate::prelude::*;
use alloc::slice;
use bevy_ecs::relationship::{RelatedSpawner, RelatedSpawnerCommands};
use core::{
    fmt::{self, Debug, Formatter},
    iter::Copied,
    marker::PhantomData,
};

#[derive(Component, Deref, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = Tasks<C>)]
pub struct TaskOf<C: CompoundTask> {
    #[deref]
    #[relationship]
    entity: Entity,
    #[reflect(ignore)]
    marker: PhantomData<C>,
}

impl<C: CompoundTask> Debug for TaskOf<C> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("TaskOf")
            .field("entity", &self.entity)
            .finish()
    }
}

impl<C: CompoundTask> Clone for TaskOf<C> {
    fn clone(&self) -> Self {
        Self {
            entity: self.entity,
            marker: PhantomData,
        }
    }
}

impl<C: CompoundTask> PartialEq for TaskOf<C> {
    fn eq(&self, other: &Self) -> bool {
        self.entity == other.entity
    }
}

impl<C: CompoundTask> Eq for TaskOf<C> {}

#[derive(Component, Deref, Reflect, Debug, Default, PartialEq, Eq)]
#[relationship_target(relationship = TaskOf<C>, linked_spawn)]
pub struct Tasks<C: CompoundTask> {
    #[deref]
    #[relationship]
    entities: Vec<Entity>,
    #[reflect(ignore)]
    marker: PhantomData<C>,
}

impl<'a, C: CompoundTask> IntoIterator for &'a Tasks<C> {
    type Item = Entity;
    type IntoIter = Copied<slice::Iter<'a, Entity>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub type TaskSpawner<'w, C> = RelatedSpawner<'w, TaskOf<C>>;

pub type TaskSpawnerCommands<'w, C> = RelatedSpawnerCommands<'w, TaskOf<C>>;

#[macro_export]
macro_rules! tasks {
    ($compound_type:ty [$($task:expr),*$(,)?]) => {
        ::bevy::prelude::related!($crate::prelude::Tasks<$compound_type>[$($task),*])
    };
}

pub use tasks;
