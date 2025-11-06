use bevy_ecs::system::SystemId;
use bevy_ecs::{lifecycle::HookContext, world::DeferredWorld};

use crate::prelude::*;

#[derive(Component)]
#[component(on_insert = Self::on_insert_hook, on_replace = Self::on_replace_hook)]
pub struct TaskSystem {
    system:
        Option<Box<dyn FnOnce(&mut Commands) -> SystemId<In<Entity>, TaskStatus> + Send + Sync>>,
    system_id: Option<SystemId<In<Entity>, TaskStatus>>,
}

impl TaskSystem {
    pub fn new<S, M>(system: S) -> Self
    where
        S: IntoSystem<In<Entity>, TaskStatus, M>,
        S::System: Send + Sync + 'static,
    {
        let system = IntoSystem::into_system(system);
        Self {
            system_id: None,
            system: Some(Box::new(move |commands| commands.register_system(system))),
        }
    }

    pub(crate) fn system_id(&self) -> SystemId<In<Entity>, TaskStatus> {
        self.system_id.unwrap()
    }

    fn on_insert_hook(mut world: DeferredWorld, context: HookContext) {
        let Some(tt) = world
            .get_mut::<Self>(context.entity)
            .and_then(|mut tt| tt.system.take())
        else {
            return;
        };
        let id = tt(&mut world.commands());
        world.get_mut::<Self>(context.entity).unwrap().system_id = Some(id);
    }

    fn on_replace_hook(mut world: DeferredWorld, context: HookContext) {
        let Some(tt) = world
            .get::<Self>(context.entity)
            .and_then(|tt| tt.system_id)
        else {
            return;
        };
        world.commands().unregister_system(tt);
    }
}
