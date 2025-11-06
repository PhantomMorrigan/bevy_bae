use core::fmt::Debug;

use bevy_ecs::system::SystemId;
use bevy_ecs::{lifecycle::HookContext, world::DeferredWorld};

use crate::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(on_insert = Self::on_insert_hook, on_replace = Self::on_replace_hook)]
pub struct TaskSystem {
    #[reflect(ignore)]
    register_system:
        Option<Box<dyn FnOnce(&mut Commands) -> SystemId<In<Entity>, TaskStatus> + Send + Sync>>,
    #[reflect(ignore)]
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
            register_system: Some(Box::new(move |commands| commands.register_system(system))),
        }
    }

    pub(crate) fn system_id(&self) -> SystemId<In<Entity>, TaskStatus> {
        self.system_id.unwrap()
    }

    fn on_insert_hook(mut world: DeferredWorld, context: HookContext) {
        let Some(register_system) = world
            .get_mut::<Self>(context.entity)
            .and_then(|mut task_system| task_system.register_system.take())
        else {
            return;
        };
        let system_id = register_system(&mut world.commands());
        world.get_mut::<Self>(context.entity).unwrap().system_id = Some(system_id);
    }

    fn on_replace_hook(mut world: DeferredWorld, context: HookContext) {
        let Some(system_id) = world
            .get::<Self>(context.entity)
            .and_then(|tt| tt.system_id)
        else {
            return;
        };
        world.commands().unregister_system(system_id);
    }
}

impl Debug for TaskSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TaskSystem")
            .field("register_system", &"<callback>")
            .field("system_id", &self.system_id)
            .finish()
    }
}
