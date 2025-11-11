//! Contains the [`Sequence`] [`CompoundTask`]

use crate::{
    plan::PlannedOperator,
    prelude::*,
    task::compound::{DecomposeId, DecomposeInput, DecomposeResult, TypeErasedCompoundTask},
};

/// A [`CompoundTask`] that decomposes into all subtasks, given that they are all valid.
#[derive(Debug, Component, Default, Reflect)]
#[reflect(Component)]
pub struct Sequence;

impl CompoundTask for Sequence {
    fn register_decompose(commands: &mut Commands) -> DecomposeId {
        commands.register_system(decompose_sequence)
    }
}

fn decompose_sequence(
    In(mut ctx): In<DecomposeInput>,
    world: &mut World,
    mut task_relations: Local<QueryState<(NameOrEntity, &Tasks)>>,
    mut individual_tasks: Local<
        QueryState<(
            Entity,
            NameOrEntity,
            AnyOf<(&Operator, &TypeErasedCompoundTask)>,
            Option<&Conditions>,
            Option<&Effects>,
        )>,
    >,
    mut conditions: Local<QueryState<(Entity, NameOrEntity, &Condition)>>,
    mut effects: Local<QueryState<(Entity, NameOrEntity, &Effect)>>,
) -> DecomposeResult {
    let Ok((name, tasks)) = task_relations.get(world, ctx.compound_task) else {
        return DecomposeResult::Failure;
    };
    let entity = ctx.compound_task;
    let seq_name = name
        .name
        .map(|n| format!("{entity} ({n})"))
        .unwrap_or_else(|| format!("{entity}"));
    debug!("sequence {seq_name}: decomposing");
    let individual_tasks: Vec<_> = individual_tasks
        .iter_many(world, tasks)
        .map(
            |(
                task_entity,
                name,
                (operator, compound_task),
                condition_relations,
                effect_relations,
            )| {
                (
                    task_entity,
                    name.name
                        .map(|n| format!("{task_entity} ({n})"))
                        .unwrap_or_else(|| format!("{task_entity}")),
                    operator.cloned(),
                    compound_task.cloned(),
                    condition_relations.cloned(),
                    effect_relations.cloned(),
                )
            },
        )
        .collect();
    let mut found_anything = false;
    for (task_entity, task_name, operator, compound_task, condition_relations, effect_relations) in
        individual_tasks
    {
        let mut individual_conditions = Vec::new();
        if let Some(condition_relations) = condition_relations {
            for (entity, name, condition) in conditions.iter_many(world, condition_relations.iter())
            {
                let name = name
                    .name
                    .map(|n| format!("{entity} ({n})"))
                    .unwrap_or_else(|| format!("{entity}"));
                let is_fulfilled = condition.is_fullfilled(&mut ctx.world_state);
                debug!(
                    "sequence {seq_name} -> task {task_name} -> condition {name}: {is_fulfilled}"
                );
                if !is_fulfilled {
                    debug!(
                        "sequence {seq_name} -> task {task_name} -> condition {name}: aborting update due to unfulfilled condition"
                    );
                    return DecomposeResult::Failure;
                }
                individual_conditions.push(condition.clone());
            }
        }
        let conditions = if !found_anything {
            // Only the first "entry" subtask needs to inherit our conditions
            ctx.conditions.extend(individual_conditions);
            ctx.conditions.clone()
        } else {
            individual_conditions
        };
        if let Some(operator) = operator {
            debug!("sequence {seq_name} -> task {task_name}: operator");
            ctx.plan.push_back(PlannedOperator {
                system: operator.system_id(),
                entity: task_entity,
                effects: vec![],
                conditions,
            });
        } else if let Some(compound_task) = compound_task {
            debug!("sequence {seq_name} -> task {task_name}: compound");

            match world.run_system_with(
                compound_task.decompose,
                DecomposeInput {
                    planner: ctx.planner,
                    compound_task: task_entity,
                    world_state: ctx.world_state.clone(),
                    plan: ctx.plan.clone(),
                    previous_mtr: ctx.previous_mtr.clone(),
                    conditions,
                },
            ) {
                Ok(DecomposeResult::Success { plan, world_state }) => {
                    ctx.plan = plan;
                    ctx.world_state = world_state;
                }
                Ok(DecomposeResult::Rejection) => return DecomposeResult::Rejection,
                Ok(DecomposeResult::Failure) | Err(_) => return DecomposeResult::Failure,
            }
        } else {
            unreachable!()
        }
        if ctx.plan.is_empty() {
            return DecomposeResult::Failure;
        }
        if let Some(effect_relations) = effect_relations {
            for (entity, name, effect) in effects.iter_many(world, effect_relations.iter()) {
                let name = name
                    .name
                    .map(|n| format!("{entity} ({n})"))
                    .unwrap_or_else(|| format!("{entity}"));
                debug!("sequence {seq_name} -> task {task_name} -> effect {name}: applied");
                effect.apply(&mut ctx.world_state);
                ctx.plan.back_mut().unwrap().effects.push(effect.clone());
            }
        }
        found_anything = true;
    }

    debug!("sequence {seq_name}: done");
    if found_anything {
        DecomposeResult::Success {
            plan: ctx.plan,
            world_state: ctx.world_state,
        }
    } else {
        DecomposeResult::Failure
    }
}
