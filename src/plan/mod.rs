//! Contains the [`Plan`] component and types for operating on it.

use alloc::collections::VecDeque;

use crate::{plan::mtr::Mtr, prelude::*, task::operator::OperatorId};

pub(crate) mod execution;
pub mod mtr;
pub mod update;

/// A full plan of operators to execute. If this is empty, either through manually clearing it, inserting it, when it runs out of operators, or fails to execute them,
/// the plan will be recomputed in the next fixed frame.
#[derive(Component, Clone, Default, PartialEq, Eq, Reflect, Debug, Deref, DerefMut)]
#[reflect(Component)]
#[require(Props)]
pub struct Plan {
    /// The queue of planned [`Operator`]s to execute. This will get [`VecDeque::pop_front`]ed during plan execution.
    #[reflect(ignore)]
    #[deref]
    pub operators_left: VecDeque<PlannedOperator>,
    /// All [`Operator`]s that were in [`Plan::operators_left`] when the plan was created.
    pub operators_total: Vec<Entity>,
    /// The [`Mtr`] of the full plan when it was created.
    pub mtr: Mtr,
}

impl Plan {
    /// Creates a new empty plan. Inserting such a plan will immediately trigger a replan at the next fixed frame.
    pub fn new() -> Self {
        Self::default()
    }

    /// Replaces the current plan with a new empty plan. Doing so will immediately trigger a replan at the next fixed frame.
    pub fn clear(&mut self) {
        *self = Self::new();
    }
}

/// An entry in [`Plan::operators_left`], representing an operator that is either currently executing or waiting to execute.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlannedOperator {
    /// The [`SystemId`](bevy_ecs::system::SystemId) of the operator.
    pub system: OperatorId,
    /// The [`Entity`] of the operator.
    pub entity: Entity,
    /// The [`Effect`]s of the operator to be applied after it completes. Does not include effects that are [`Effect::plan_only`].
    /// The last operator of a compound task will also inherit effects from higher-up compound tasks.
    pub effects: Vec<Effect>,
    /// The [`Condition`]s that need to be fulfilled for the operator to be run.
    /// The first operator of a compound task will also inherit conditions from higher-up compound tasks.
    pub conditions: Vec<Condition>,
}
