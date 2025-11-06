use core::marker::PhantomData;

use bevy_ecs::system::SystemId;

use crate::prelude::*;

#[derive(Component)]
pub struct FuncCondition<
    M: Send + Sync + 'static,
    I: IntoSystem<In<Entity>, bool, M> + Send + Sync + 'static,
> {
    system: Option<I>,
    system_id: Option<SystemId<In<Entity>, bool>>,
    marker: PhantomData<M>,
}

impl<M: Send + Sync + 'static, I: IntoSystem<In<Entity>, bool, M> + Send + Sync + 'static>
    FuncCondition<M, I>
{
    pub fn new(system: I) -> Self {
        Self {
            system: Some(system),
            system_id: None,
            marker: PhantomData,
        }
    }
}

impl<M: Send + Sync + 'static, I: IntoSystem<In<Entity>, bool, M> + Send + Sync + 'static> Condition
    for FuncCondition<M, I>
{
    fn is_satisfied(&mut self, world: EntityWorldMut) -> bool {
        let entity = world.id();
        let world = world.into_world_mut();
        let system_id = match self.system_id {
            Some(id) => id,
            None => {
                let id = world.register_system(self.system.take().unwrap());
                self.system_id = Some(id);
                id
            }
        };
        world.run_system_with(system_id, entity).unwrap()
    }
}
