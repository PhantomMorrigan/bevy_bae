use bevy_ecs::error::ErrorContext;
use bevy_mod_props::PropsExt;

use crate::plan::PlannedOperator;
use crate::prelude::*;
use crate::task::compound::{DecomposeInput, DecomposeResult, TypeErasedCompoundTask};

#[derive(EntityEvent)]
struct UpdatePlan {
    #[event_target]
    entity: Entity,
}

pub struct UpdatePlanCommand;

impl EntityCommand for UpdatePlanCommand {
    fn apply(self, entity_world: EntityWorldMut) {
        let entity = entity_world.id();
        let world = entity_world.into_world_mut();
        let error_handler = world.default_error_handler();
        let result: Result<(), _> =
            world.run_system_cached_with(update_plan, UpdatePlan { entity });
        match result {
            Ok(_) => (),
            Err(bevy_ecs::system::RegisteredSystemError::Failed(err)) => (error_handler)(
                err,
                ErrorContext::Command {
                    name: DebugName::from("UpdatePlanCommand"),
                },
            ),
            Err(err) => {
                panic!("Unexpected error while calling `update_plan`: {err}")
            }
        }
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
    mut conditions: Local<QueryState<(Entity, NameOrEntity, &Condition)>>,
    mut effects: Local<QueryState<(Entity, NameOrEntity, &Effect)>>,
    mut tasks: Local<QueryState<AnyOf<(&Operator, &TypeErasedCompoundTask)>>>,
    mut names: Local<QueryState<NameOrEntity>>,
) -> Result {
    let root = update.entity;
    let behav_name = names
        .get(world, root)
        .ok()
        .and_then(|name| name.name.map(|n| format!("{root} ({n})")))
        .unwrap_or_else(|| format!("{root}"));

    debug!("behavior {behav_name}: Updating plan");

    let mut world_state = world.entity(update.entity).props().clone();
    if let Some(condition_relations) = world.get::<Conditions>(root) {
        for (entity, name, condition) in conditions.iter_many(world, condition_relations) {
            let name = name
                .name
                .map(|n| format!("{entity} ({n})"))
                .unwrap_or_else(|| format!("{entity}"));
            let is_fulfilled = condition.is_fullfilled(&mut world_state);
            debug!("behavior {behav_name} -> condition {name}: {is_fulfilled}");
            if !is_fulfilled {
                debug!("behavior {behav_name}: aborting update due to unfulfilled condition");
                world.entity_mut(root).insert(Plan::default());
                return Ok(());
            }
        }
    }

    let Ok((operator, task)) = tasks
        .get(world, root)
        .map(|(o, t)| (o.cloned(), t.cloned()))
    else {
        world.entity_mut(root).insert(Plan::default());
        return Err(BevyError::from(format!(
            "{behav_name}: Called `update_plan` for an entity without any tasks. Ensure it has either an `Operator` or a `CompoundTask` like `Select` or `Sequence`"
        )));
    };
    let mut plan = if let Some(operator) = operator {
        // well that was easy: this root has just a single operator
        debug!("behavior {behav_name}: operator");
        Plan(
            [PlannedOperator {
                system: operator.system_id(),
                entity: root,
                effects: vec![],
            }]
            .into(),
        )
    } else if let Some(compound_task) = task {
        debug!("behavior {behav_name}: compound task");
        let ctx = DecomposeInput {
            world_state,
            plan: Plan::default(),
            root,
            compound_task: root,
        };
        let result = world.run_system_with(compound_task.decompose, ctx)?;
        match result {
            DecomposeResult::Success { plan, .. } => plan,
            DecomposeResult::Failure => Plan::default(),
            DecomposeResult::Rejection => todo!(),
        }
    } else {
        unreachable!(
            "Bevy should guarantee that `AnyOf` contains at least one element that is `Some`"
        )
    };

    if !plan.is_empty()
        && let Some(effect_relations) = world.get::<Effects>(root)
    {
        for (entity, name, effect) in effects.iter_many(world, effect_relations) {
            let name = name
                .name
                .map(|n| format!("{entity} ({n})"))
                .unwrap_or_else(|| format!("{entity}"));
            debug!("behavior {behav_name} -> effect {name}: queued");
            plan.back_mut().unwrap().effects.push(effect.clone());
        }
    }
    debug!("behavior {behav_name}: finished with {plan:?}");

    world.entity_mut(root).insert(plan);
    Ok(())
}
