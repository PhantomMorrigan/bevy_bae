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
        task::{
            TaskStatus,
            compound::{select::Select, sequence::Sequence},
            relationship::{TaskOf, TaskSpawner, TaskSpawnerCommands, Tasks, tasks},
            step::{ExecuteStep, Step},
        },
    };
    pub(crate) use {
        bevy_app::prelude::*, bevy_derive::Deref, bevy_ecs::prelude::*, bevy_reflect::prelude::*,
    };
}
extern crate alloc;

use crate::prelude::*;

pub mod condition;
pub mod effect;
pub mod task;

pub struct BaePlugin;
impl Plugin for BaePlugin {
    fn build(&self, app: &mut App) {}
}
