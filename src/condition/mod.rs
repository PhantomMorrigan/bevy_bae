use bevy_ecs::system::SystemId;
use bevy_ecs::{lifecycle::HookContext, world::DeferredWorld};
use core::marker::PhantomData;

use crate::prelude::*;

pub mod relationship;

#[derive(Component)]
pub struct RegisteredCondition {
    pub system_id: SystemId<In<Entity>, bool>,
}

#[derive(Component)]
#[component(on_add = Condition::<I, M>::queue_into_condition)]
pub struct Condition<
    I: IntoSystem<In<Entity>, bool, M> + Send + Sync + 'static,
    M: Send + Sync + 'static,
> {
    system: Option<I>,
    marker: PhantomData<M>,
}

pub trait IntoCondition {
    type System: IntoSystem<In<Entity>, bool, Self::Marker> + Send + Sync + 'static;
    type Marker: Send + Sync + 'static;

    fn into_condition(self) -> Condition<Self::System, Self::Marker>;
}

impl<M: Send + Sync + 'static, I: IntoSystem<In<Entity>, bool, M> + Send + Sync + 'static>
    Condition<I, M>
{
    pub fn new(system: I) -> Self {
        Self {
            system: Some(system),
            marker: PhantomData,
        }
    }

    fn queue_into_condition(mut world: DeferredWorld, ctx: HookContext) {
        let entity = ctx.entity;
        world.commands().queue(move |world: &mut World| -> Result {
            if world.get_entity(entity).is_err() {
                // Already despawned
                return Ok(());
            }
            let system = {
                let mut entity_world = world.entity_mut(entity);
                let Some(mut func_condition) = entity_world.get_mut::<Condition<I, M>>() else {
                    // Already removed
                    return Ok(());
                };
                func_condition.system.take().unwrap()
            };
            let system_id = world.register_system(system);
            world
                .entity_mut(entity)
                .insert(RegisteredCondition { system_id })
                .remove::<Condition<I, M>>();

            Ok(())
        });
    }
}

impl<M: Send + Sync + 'static, I: IntoSystem<In<Entity>, bool, M> + Send + Sync + 'static>
    IntoCondition for Condition<I, M>
{
    type Marker = M;
    type System = I;

    fn into_condition(self) -> Condition<Self::System, Self::Marker> {
        self
    }
}
