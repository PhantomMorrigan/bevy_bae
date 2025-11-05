use std::marker::PhantomData;

use bevy_ecs::system::IntoObserverSystem;

use crate::prelude::*;

#[derive(Component)]
pub struct Step<M, I: IntoObserverSystem<ExecuteStep, (), M, TaskStatus>> {
    observer: I,
    marker: PhantomData<M>,
}

impl<M, I: IntoObserverSystem<ExecuteStep, (), M, TaskStatus>> Step<M, I> {
    pub fn new(observer: I) -> Self {
        Self {
            observer,
            marker: PhantomData,
        }
    }
}

#[derive(EntityEvent, Deref)]
pub struct ExecuteStep {
    #[deref]
    #[event_target]
    pub entity: Entity,
}

impl From<Entity> for ExecuteStep {
    fn from(entity: Entity) -> Self {
        Self { entity }
    }
}
