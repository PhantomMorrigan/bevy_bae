//! Contains the [`Plan`] component and types for operating on it.

use alloc::collections::VecDeque;
use bevy_ecs::{entity_disabling::Disabled, query::QueryEntityError};

use crate::{plan::mtr::Mtr, prelude::*};

pub(crate) mod execution;
pub mod mtr;
pub mod update;

/// A full plan of operators to execute. If this is empty, either through manually clearing it, inserting it, when it runs out of operators, or fails to execute them,
/// the plan will be recomputed in the next fixed frame.
#[derive(Component, Clone, Default, PartialEq, Eq, Reflect, Debug, Deref, DerefMut)]
#[reflect(Component)]
#[require(Props)]
pub struct Plan {
    /// The queue of planned [`TaskNodes`]s to execute. This will get [`VecDeque::pop_front`]ed during plan execution.
    #[reflect(ignore)]
    #[deref]
    pub operators_left: VecDeque<usize>,
    /// Currently executing tasks.
    pub track: Vec<usize>,
    /// All [`Operator`]s that were in [`Plan::operators_left`] when the plan was createdk
    pub nodes: Vec<TaskNode>,
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

    pub(crate) fn add_node(&mut self, node: TaskNode) -> usize {
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    pub(crate) fn merge(&mut self, mut other: Plan) {
        let len = self.nodes.len();
        self.nodes.append(&mut other.nodes);
        for idx in other.operators_left {
            self.push_back(idx + len);
        }
    }
}

/// An entry in [`Plan::operators_left`], representing an operator that is either currently executing or waiting to execute.
#[derive(Clone, Debug, PartialEq, Eq, Reflect)]
pub struct TaskNode {
    /// The [`Entity`] of the [`Operator`].
    pub entity: Entity,
    /// Whenever or not this has an associated [`Operator`]
    pub composite: bool,
    /// The last operator of a compound task will also inherit effects from higher-up compound tasks.
    pub effects: Vec<Entity>,
    /// The [`Condition`]s that need to be fulfilled for the operator to be run.
    /// The first operator of a compound task will also inherit conditions from higher-up compound tasks.
    pub conditions: Vec<Entity>,
}

/// An [`EntityEvent`] for logging a given plan via [`info!`]
#[derive(EntityEvent, Debug)]
pub struct LogPlan {
    entity: Entity,
}

impl LogPlan {
    /// Creates a new [`LogPlan`] event for the given entity.
    pub fn new(entity: Entity) -> Self {
        LogPlan { entity }
    }
}

impl From<Entity> for LogPlan {
    fn from(entity: Entity) -> Self {
        LogPlan::new(entity)
    }
}

pub(crate) fn log_plan(
    log: On<LogPlan>,
    plans: Query<&Plan, Allow<Disabled>>,
    names: Query<NameOrEntity, Allow<Disabled>>,
) -> Result {
    let plan_entity = log.entity;
    let plan = plans.get(plan_entity)?;
    let name = |entity| -> Result<String, QueryEntityError> {
        names.get(entity).map(|n| n.entity_and_name())
    };
    let plan_name = name(plan_entity)?;
    let mut log = String::new();
    log.push_str(&format!("plan {plan_name}:\n"));
    log.push_str(&format!("- mtr: {}\n", plan.mtr));
    log.push_str(&format!(
        "- operators left ({}):\n",
        plan.operators_left.len()
    ));
    for operator in plan.operators_left.iter().map(|i| &plan.nodes[*i]) {
        let operator_name = name(operator.entity)?;
        log.push_str(&format!("  - {operator_name}:\n"));
        log.push_str(&format!("    - effects ({}):\n", operator.effects.len()));
        for effect in &operator.effects {
            let effect_name = name(*effect)?;
            log.push_str(&format!("      - {effect_name}\n"));
        }
        log.push_str(&format!(
            "    - conditions ({}):\n",
            operator.conditions.len()
        ));
        for condition in &operator.conditions {
            let condition_name = name(*condition)?;
            log.push_str(&format!("      - {condition_name}\n"));
        }
    }
    log.push_str(&format!("- total operators ({})\n", plan.nodes.len()));
    for operator in &plan.nodes {
        let operator_name = name(operator.entity)?;
        log.push_str(&format!("  - {operator_name}\n"));
    }
    info!("{}", log.trim());
    Ok(())
}
