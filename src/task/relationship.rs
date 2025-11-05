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
pub struct TaskOf<C: Component> {
    #[deref]
    #[relationship]
    entity: Entity,
    #[reflect(ignore)]
    marker: PhantomData<C>,
}

impl<C: Component> Debug for TaskOf<C> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("TaskOf")
            .field("entity", &self.entity)
            .finish()
    }
}

impl<C: Component> Clone for TaskOf<C> {
    fn clone(&self) -> Self {
        Self {
            entity: self.entity,
            marker: PhantomData,
        }
    }
}

impl<C: Component> PartialEq for TaskOf<C> {
    fn eq(&self, other: &Self) -> bool {
        self.entity == other.entity
    }
}

impl<C: Component> Eq for TaskOf<C> {}

#[derive(Component, Deref, Reflect, Debug, Default, PartialEq, Eq)]
#[relationship_target(relationship = TaskOf<C>, linked_spawn)]
pub struct Tasks<C: Component> {
    #[deref]
    #[relationship]
    entities: Vec<Entity>,
    #[reflect(ignore)]
    marker: PhantomData<C>,
}

impl<'a, C: Component> IntoIterator for &'a Tasks<C> {
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
    ($task_kind:ty [$($task:expr),*$(,)?]) => {
        ::bevy::prelude::related!($crate::prelude::Tasks<$task_kind>[$($task),*])
    };
    ($action:expr) => {
        $crate::prelude::tasks!($crate::prelude::Step<_, _> [$action])
    };
}

pub use tasks;
