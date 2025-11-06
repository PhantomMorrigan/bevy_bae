pub mod prelude {
    pub use crate::{
        condition::{
            Condition,
            relationship::{
                ConditionOf, ConditionSpawner, ConditionSpawnerCommands, Conditions,
                IntoConditionBundle, conditions,
            },
        },
        effect::{
            Effect,
            relationship::{
                EffectOf, EffectSpawner, EffectSpawnerCommands, Effects, IntoEffectBundle, effects,
            },
        },
        plan::Planner,
        task::{
            TaskStatus,
            compound::{
                CompoundTask,
                relationship::{
                    BaeTaskOf, BaeTaskSpawner, BaeTaskSpawnerCommands, BaeTasks, IntoTaskBundle,
                    tasks,
                },
                select::Select,
                sequence::Sequence,
            },
            primitive::Operator,
        },
    };
    pub use bevy_mod_props::{self, Props, Value};
    pub(crate) use {
        crate::value_ext::ValueExt as _, bevy_app::prelude::*, bevy_derive::Deref,
        bevy_ecs::prelude::*, bevy_reflect::prelude::*, log::warn,
    };
}
extern crate alloc;

use bevy_ecs::{intern::Interned, schedule::ScheduleLabel};

use crate::{
    prelude::*,
    task::{
        compound::CompoundAppExt,
        validation::{insert_bae_task_present_on_add, remove_bae_task_present_on_remove},
    },
};

pub mod condition;
pub mod effect;
pub mod plan;
pub mod task;
mod value_ext;

pub struct BaePlugin {
    schedule: Interned<dyn ScheduleLabel>,
}

impl Default for BaePlugin {
    fn default() -> Self {
        Self {
            schedule: FixedUpdate.intern(),
        }
    }
}
impl Plugin for BaePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(self.schedule, (BaeSystems::RunTaskSystems,).chain());
        app.world_mut().register_component::<Condition>();
        app.world_mut().register_component::<Effect>();
        app.add_observer(insert_bae_task_present_on_add::<Operator>)
            .add_observer(remove_bae_task_present_on_remove::<Operator>);
        app.add_compound_task::<Select>()
            .add_compound_task::<Sequence>();
    }
}

#[derive(SystemSet, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum BaeSystems {
    RunTaskSystems,
}
