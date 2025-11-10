pub mod prelude {
    pub use crate::{
        BaePlugin, BaeSystems,
        condition::{
            Condition,
            relationship::{
                ConditionOf, ConditionSpawner, ConditionSpawnerCommands, Conditions, conditions,
            },
        },
        effect::{
            Effect,
            relationship::{EffectOf, EffectSpawner, EffectSpawnerCommands, Effects, effects},
        },
        plan::{
            Plan,
            update::{UpdatePlanCommand, UpdatePlanCommands as _},
        },
        task::{
            TaskStatus,
            compound::{
                CompoundTask,
                relationship::{TaskOf, TaskSpawner, TaskSpawnerCommands, Tasks, tasks},
                select::Select,
                sequence::Sequence,
            },
            primitive::{Operator, OperatorInput},
        },
    };
    pub use bevy_mod_props::{self, Props, Value};
    pub(crate) use {
        bevy_app::prelude::*,
        bevy_derive::Deref,
        bevy_ecs::prelude::*,
        bevy_reflect::prelude::*,
        bevy_utils::prelude::*,
        log::{debug, error},
    };
}
extern crate alloc;

use bevy_ecs::{intern::Interned, schedule::ScheduleLabel};

use crate::{
    plan::execution::execute_plan,
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
        app.add_systems(
            self.schedule,
            execute_plan.in_set(BaeSystems::RunTaskSystems),
        );
    }
}

#[derive(SystemSet, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum BaeSystems {
    RunTaskSystems,
}
