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
            compound::{
                CompoundTask,
                relationship::{TaskOf, TaskSpawner, TaskSpawnerCommands, Tasks, tasks},
                select::Select,
                sequence::Sequence,
            },
            primitive::TaskSystem,
        },
    };
    pub use bevy_mod_props::{self, Props, Value};
    pub(crate) use {
        crate::value_ext::ValueExt as _, bevy_app::prelude::*, bevy_derive::Deref,
        bevy_ecs::prelude::*, bevy_reflect::prelude::*,
    };
}
extern crate alloc;

use crate::prelude::*;

pub mod condition;
pub mod effect;
pub mod task;
mod value_ext;

pub struct BaePlugin;
impl Plugin for BaePlugin {
    fn build(&self, _app: &mut App) {}
}
