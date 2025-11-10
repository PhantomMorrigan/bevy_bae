use crate::prelude::*;

pub(crate) fn execute_plan(
    world: &mut World,
    mut plans: Local<QueryState<(Entity, NameOrEntity, &mut Plan)>>,
) {
    let plans = plans
        .iter(world)
        .filter_map(|(entity, name, plan)| {
            Some((
                entity,
                name.name
                    .map(|n| format!("{entity} ({n})"))
                    .unwrap_or_else(|| format!("{entity}")),
                plan.front()?.clone(),
            ))
        })
        .collect::<Vec<_>>();
    for (entity, name, planned_operator) in plans {
        debug!("{name}: Executing plan");
        let input = OperatorInput {
            planner: entity,
            operator: planned_operator.entity,
        };
        let result: Result<TaskStatus, _> = world.run_system_with(planned_operator.system, input);
        match result {
            Ok(TaskStatus::Success) => {
                debug!("{name}: Plan step completed successfully, moving to next step");
                let step = world
                    .entity_mut(entity)
                    .get_mut::<Plan>()
                    .unwrap()
                    .pop_front()
                    .unwrap();

                if !world.entity_mut(entity).contains::<Props>() {
                    world.entity_mut(entity).insert(Props::default());
                }
                let mut entity = world.entity_mut(entity);
                let mut props = entity.get_mut::<Props>().unwrap();
                for effect in step.effects {
                    debug!("{name}: applied effect");
                    effect.apply(&mut props);
                }
            }
            Ok(TaskStatus::Continue) => {
                debug!("{name}: Plan step ongoing.");
            }
            Ok(TaskStatus::Failure) => {
                debug!("{name}: Plan step failed, aborting plan");
                world.entity_mut(entity).insert(Plan::default());
            }
            Err(err) => {
                error!("{name}: failed to execute current plan step: {err}. Aborting plan");
                world.entity_mut(entity).insert(Plan::default());
            }
        }
        if world
            .entity(entity)
            .get::<Plan>()
            .is_none_or(|plan| plan.is_empty())
        {
            debug!("{name}: Plan is empty, triggering replan.");
            world.entity_mut(entity).trigger(UpdatePlan::new);
        }
    }
}
