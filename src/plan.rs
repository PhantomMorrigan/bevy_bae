use bevy_mod_props::PropsExt;

use crate::prelude::*;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Planner;

#[derive(EntityEvent)]
pub struct UpdatePlan {
    #[event_target]
    entity: Entity,
}

pub(crate) fn update_plan(
    update: On<UpdatePlan>,
    world: &World,
    mut conditions: Local<QueryState<&Condition>>,
) -> Result {
    let root = update.entity;
    let planner = world
        .entity(root)
        .get::<Planner>()
        .ok_or("`UpdatePlan` was called on an entity without a `Planner`")?;
    let mut world_state = world.entity(update.entity).props().clone();

    let mut tasks_to_process = vec![root];
    while let Some(task_entity) = tasks_to_process.pop() {
        if let Some(condition_relations) = world.get::<Conditions>(task_entity) {
            for condition in conditions.iter_many(world, condition_relations) {
                if !condition.is_fullfilled(&mut world_state) {
                    // RestoreToLastDecomposedTask
                }
            }
        }
    }

    Ok(())
}
