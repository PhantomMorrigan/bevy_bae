use bevy_mod_props::PropsExt;

use crate::prelude::*;
use crate::task::compound::{DecomposeInput, DecomposeResult, TypeErasedCompoundTask};
use crate::task::primitive::OperatorId;

#[derive(EntityEvent)]
struct UpdatePlan {
    #[event_target]
    entity: Entity,
}

#[derive(Component, Clone, PartialEq, Eq, Reflect, Debug)]
#[reflect(Component)]
pub struct Plan(#[reflect(ignore)] pub Vec<OperatorId>);

pub struct UpdatePlanCommand;

impl EntityCommand for UpdatePlanCommand {
    fn apply(self, entity_world: EntityWorldMut) {
        let entity = entity_world.id();
        entity_world
            .into_world_mut()
            .run_system_cached_with(update_plan, UpdatePlan { entity })
            .unwrap()
    }
}

pub trait UpdatePlanCommands {
    fn update_plan(&mut self) -> &mut Self;
}

impl<'a> UpdatePlanCommands for EntityCommands<'a> {
    fn update_plan(&mut self) -> &mut Self {
        self.queue(UpdatePlanCommand)
    }
}

fn update_plan(
    update: In<UpdatePlan>,
    world: &mut World,
    mut conditions: Local<QueryState<&Condition>>,
    mut tasks: Local<QueryState<AnyOf<(&Operator, &TypeErasedCompoundTask)>>>,
    mut names: Local<QueryState<NameOrEntity>>,
) -> Result {
    let root = update.entity;

    let mut world_state = world.entity(update.entity).props().clone();
    if let Some(condition_relations) = world.get::<Conditions>(root) {
        let is_fulfilled = conditions
            .iter_many(world, condition_relations)
            .all(|condition| condition.is_fullfilled(&mut world_state));
        if !is_fulfilled {
            return Ok(());
        }
    }

    let Ok((operator, task)) = tasks
        .get(world, root)
        .map(|(o, t)| (o.cloned(), t.cloned()))
    else {
        let name = names
            .get(world, root)
            .ok()
            .and_then(|name| name.name.map(|n| format!("{root} ({n})")))
            .unwrap_or_else(|| format!("{root}"));
        return Err(BevyError::from(format!(
            "{name}: Called `UpdatePlan` for an entity without any tasks. Ensure it has either an `Operator` or a `CompoundTask` like `Select` or `Sequence`"
        )));
    };
    let plan = if let Some(operator) = operator {
        // well that was easy: this root has just a single operator
        vec![operator.system_id()]
    } else if let Some(compound_task) = task {
        let ctx = DecomposeInput {
            world_state,
            plan: vec![],
            root,
            compound_task: root,
        };
        let result = world.run_system_with(compound_task.decompose, ctx)?;
        match result {
            DecomposeResult::Success { plan, .. } => plan,
            DecomposeResult::Failure => {
                todo!();
            }
            DecomposeResult::Rejection => todo!(),
        }
    } else {
        unreachable!(
            "Bevy should guarantee that `AnyOf` contains at least one element that is `Some`"
        )
    };

    // No need to apply the effects of the root, as they cannot affect any planning.
    // But if we ever decided to automatically apply effects to the real props, we should put that here!

    world.entity_mut(root).insert(Plan(plan));
    Ok(())
}
