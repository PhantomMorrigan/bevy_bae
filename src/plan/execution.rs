use crate::prelude::*;

pub(crate) fn update_empty_plans(
    mut plans: Query<(Entity, NameOrEntity, &Plan)>,
    mut commands: Commands,
) {
    for (entity, name, plan) in plans.iter_mut() {
        if plan.is_empty() {
            commands.entity(entity).trigger(UpdatePlan::new);
            let name = name
                .name
                .map(|n| format!("{} ({}))", name.entity, n))
                .unwrap_or_else(|| format!("{}", name.entity));
            debug!("{name}: Plan is empty, triggering replan.");
        }
    }
}
pub(crate) fn execute_plan(
    world: &mut World,
    mut plans: Local<QueryState<(Entity, NameOrEntity, &mut Plan)>>,
    mut conditions: Local<QueryState<&Condition>>,
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
        if !world.entity_mut(entity).contains::<Props>() {
            world.entity_mut(entity).insert(Props::default());
        }
        debug!("{name}: validating plan");
        let conditions = {
            let task_entity = world.entity(planned_operator.entity);
            task_entity
                .get::<Conditions>()
                .iter()
                .flat_map(|c| {
                    conditions
                        .iter_many(world, c.iter())
                        .cloned()
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        };
        let mut all_conditions_met = true;
        {
            let mut entity = world.entity_mut(entity);
            let mut props = entity.get_mut::<Props>().unwrap();
            for condition in conditions {
                if condition.is_fullfilled(&mut props) {
                    debug!("{name}: Condition met");
                } else {
                    debug!("{name}: Condition not met. Aborting plan");
                    all_conditions_met = false;
                    break;
                }
            }
        }
        let result: Result<OperatorStatus, _> = if all_conditions_met {
            debug!("{name}: executing plan step");
            let input = OperatorInput {
                planner: entity,
                operator: planned_operator.entity,
            };
            world.run_system_with(planned_operator.system, input)
        } else {
            Ok(OperatorStatus::Failure)
        };

        let force_replan = match result {
            Ok(OperatorStatus::Success) => {
                debug!("{name}: Plan step completed successfully, moving to next step");
                let step = world
                    .entity_mut(entity)
                    .get_mut::<Plan>()
                    .unwrap()
                    .pop_front()
                    .unwrap();

                let mut entity = world.entity_mut(entity);
                let mut props = entity.get_mut::<Props>().unwrap();
                for effect in step.effects {
                    if effect.plan_only {
                        debug!("{name}: skipped plan-only effect");
                    } else {
                        debug!("{name}: applied effect");
                        effect.apply(&mut props);
                    }
                }

                false
            }
            Ok(OperatorStatus::Ongoing) => {
                debug!("{name}: Plan step ongoing.");
                // Even if the current plan is empty, we still want to continue the execution of the last step!
                continue;
            }
            Ok(OperatorStatus::Failure) => {
                debug!("{name}: Plan step failed, aborting plan");
                true
            }
            Err(err) => {
                error!("{name}: failed to execute current plan step: {err}. Aborting plan");
                true
            }
        };
        if force_replan
            || world
                .entity(entity)
                .get::<Plan>()
                .is_none_or(|plan| plan.is_empty())
        {
            world.entity_mut(entity).insert(Plan::default());
            debug!("{name}: forcing a replan.");
        }
    }
}
