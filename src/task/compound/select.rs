//! Contains the [`Select`] [`CompoundTask`]

use crate::{
    plan::PlannedOperator,
    prelude::*,
    task::compound::{DecomposeId, DecomposeInput, DecomposeResult, TypeErasedCompoundTask},
};

/// A [`CompoundTask`] that decomposes into the first valid subtask.
#[derive(Debug, Component, Default, Reflect)]
#[reflect(Component)]
pub struct Select;

impl CompoundTask for Select {
    fn register_decompose(commands: &mut Commands) -> DecomposeId {
        commands.register_system(decompose_select)
    }
}

fn decompose_select(
    In(mut ctx): In<DecomposeInput>,
    world: &mut World,
    mut task_relations: Local<QueryState<&Tasks>>,
    mut individual_tasks: Local<
        QueryState<
            (
                Entity,
                Has<Operator>,
                Option<&TypeErasedCompoundTask>,
                Option<&Conditions>,
                Option<&Effects>,
            ),
            Or<(With<Operator>, With<TypeErasedCompoundTask>)>,
        >,
    >,
    mut conditions: Local<QueryState<(Entity, &Condition)>>,
    mut effects: Local<QueryState<(Entity, &Effect)>>,
    mut individual_tasks_scratch: Local<
        Vec<(
            Entity,
            bool,
            Option<TypeErasedCompoundTask>,
            Option<Conditions>,
            Option<Effects>,
        )>,
    >,
) -> DecomposeResult {
    let Ok(tasks) = task_relations.get(world, ctx.compound_task) else {
        return DecomposeResult::Failure;
    };
    individual_tasks_scratch.extend(individual_tasks.iter_many(world, tasks).map(
        |(task_entity, has_operator, compound_task, condition_relations, effect_relations)| {
            (
                task_entity,
                has_operator,
                compound_task.cloned(),
                condition_relations.cloned(),
                effect_relations.cloned(),
            )
        },
    ));

    'task: for (
        i,
        (task_entity, has_operator, compound_task, condition_relations, effect_relations),
    ) in individual_tasks_scratch.drain(..).enumerate()
    {
        let mtr = ctx.plan.mtr.clone().with(i as u16);
        if mtr > ctx.previous_mtr {
            return DecomposeResult::Rejection;
        }
        if let Some(condition_relations) = condition_relations {
            for (entity, condition) in conditions.iter_many(world, condition_relations.iter()) {
                if !condition.is_fullfilled(&mut ctx.world_state) {
                    continue 'task;
                }
                ctx.conditions.push(entity);
            }
        }
        if has_operator {
            ctx.plan.push_back(PlannedOperator {
                entity: task_entity,
                effects: vec![],
                conditions: ctx.conditions.clone(),
            });
        } else if let Some(compound_task) = compound_task {
            let result = world.run_system_with(
                compound_task.decompose,
                DecomposeInput {
                    planner: ctx.planner,
                    compound_task: task_entity,
                    world_state: ctx.world_state.clone(),
                    plan: ctx.plan.clone(),
                    previous_mtr: ctx.previous_mtr.clone(),
                    conditions: ctx.conditions.clone(),
                },
            );
            world.flush();
            match result {
                Ok(DecomposeResult::Success { plan, world_state }) => {
                    ctx.plan = plan;
                    ctx.world_state = world_state;
                }
                Ok(DecomposeResult::Rejection) => return DecomposeResult::Rejection,
                Ok(DecomposeResult::Failure) | Err(_) => continue,
            }
        } else {
            unreachable!()
        }
        if ctx.plan.is_empty() {
            return DecomposeResult::Failure;
        }
        if let Some(effect_relations) = effect_relations {
            for (entity, effect) in effects.iter_many(world, effect_relations.iter()) {
                effect.apply(&mut ctx.world_state);
                ctx.plan.back_mut().unwrap().effects.push(entity);
            }
        }
        // only use the first match
        ctx.plan.mtr.push(i as u16);
        return DecomposeResult::Success {
            plan: ctx.plan,
            world_state: ctx.world_state,
        };
    }
    DecomposeResult::Failure
}
