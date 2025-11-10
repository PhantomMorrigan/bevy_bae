use crate::{
    prelude::*,
    task::compound::{DecomposeId, DecomposeInput, DecomposeResult, TypeErasedCompoundTask},
};

#[derive(Debug, Default, Reflect)]
pub struct Select;

impl CompoundTask for Select {
    fn register_decompose(commands: &mut Commands) -> DecomposeId {
        commands.register_system(decompose_select)
    }
}

fn decompose_select(
    In(mut ctx): In<DecomposeInput>,
    world: &mut World,
    mut task_relations: Local<QueryState<(NameOrEntity, &Tasks<Select>)>>,
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
    let sel_name = name
        .name
        .map(|n| format!("{entity} ({n})"))
        .unwrap_or_else(|| format!("{entity}"));
    debug!("select {sel_name}: decomposing");
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
    'task: for (
        task_entity,
        task_name,
        operator,
        compound_task,
        condition_relations,
        effect_relations,
    ) in individual_tasks
    {
        if let Some(condition_relations) = condition_relations {
            for (entity, name, condition) in conditions.iter_many(world, condition_relations.iter())
            {
                let name = name
                    .name
                    .map(|n| format!("{entity} ({n})"))
                    .unwrap_or_else(|| format!("{entity}"));
                let is_fulfilled = condition.is_fullfilled(&mut ctx.world_state);
                debug!("select {sel_name} -> task {task_name} -> condition {name}: {is_fulfilled}");
                if !is_fulfilled {
                    debug!(
                        "select {sel_name} -> task {task_name} -> condition {name}: skipping due to unfulfilled condition"
                    );
                    continue 'task;
                }
            }
        }
        if let Some(operator) = operator {
            debug!("select {sel_name} -> task {task_name}: operator");
            ctx.plan.push((operator.system_id(), vec![]));
        } else if let Some(compound_task) = compound_task {
            debug!("select {sel_name} -> task {task_name}: compound");
            match world.run_system_with(
                compound_task.decompose,
                DecomposeInput {
                    root: ctx.root,
                    compound_task: task_entity,
                    world_state: ctx.world_state.clone(),
                    plan: ctx.plan.clone(),
                },
            ) {
                Ok(DecomposeResult::Success { plan, world_state }) => {
                    ctx.plan = plan;
                    ctx.world_state = world_state;
                }
                Ok(DecomposeResult::Failure) => continue,
                Ok(DecomposeResult::Rejection) => todo!(),
                Err(_) => continue,
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
                debug!("select {sel_name} -> task {task_name} -> effect {name}: applied");
                effect.apply(&mut ctx.world_state);
                ctx.plan.last_mut().unwrap().1.push(effect.clone());
            }
        }
        // only use the first match
        found_anything = true;
        break;
    }
    if found_anything {
        debug!("select {sel_name}: done");
        DecomposeResult::Success {
            plan: ctx.plan,
            world_state: ctx.world_state,
        }
    } else {
        DecomposeResult::Failure
    }
}
