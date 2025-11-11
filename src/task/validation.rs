use bevy_ecs::entity_disabling::Disabled;

use crate::prelude::*;

#[derive(Component, Debug, Default)]
pub(crate) struct BaeTaskPresent(bool);

pub(crate) fn insert_bae_task_present_on_add<T: Component>(
    add: On<Add, T>,
    mut present: Query<&mut BaeTaskPresent, Allow<Disabled>>,
    names: Query<NameOrEntity>,
) {
    let id = add.entity;
    let Ok(mut present) = present.get_mut(id) else {
        return;
    };
    if present.0 {
        let name = names
            .get(add.entity)
            .ok()
            .and_then(|name| name.name.map(|_| format!("{id} ({name})")))
            .unwrap_or_else(|| format!("{id}"));
        panic!("Entity {name} holds more than one type of task");
    }
    present.0 = true;
}

pub(crate) fn remove_bae_task_present_on_remove<T: Component>(
    remove: On<Remove, T>,
    mut commands: Commands,
) {
    commands
        .entity(remove.entity)
        .try_remove::<BaeTaskPresent>();
}

#[cfg(test)]
mod tests {
    use bevy::MinimalPlugins;

    use crate::BaePlugin;

    use super::*;

    #[test]
    #[should_panic]
    fn panics_on_compound_and_primitive() {
        App::new()
            .add_plugins((MinimalPlugins, BaePlugin::default()))
            .add_systems(Startup, |mut commands: Commands| {
                commands.spawn((
                    Tasks::default(),
                    Operator::new(|_: In<OperatorInput>| OperatorStatus::Success),
                ));
            })
            .update();
    }

    #[test]
    fn does_not_panic_on_primitive() {
        App::new()
            .add_plugins((MinimalPlugins, BaePlugin::default()))
            .add_systems(Startup, |mut commands: Commands| {
                commands.spawn(Operator::new(|_: In<OperatorInput>| {
                    OperatorStatus::Success
                }));
            })
            .update();
    }

    #[test]
    fn does_not_panic_on_compound() {
        App::new()
            .add_plugins((MinimalPlugins, BaePlugin::default()))
            .add_systems(Startup, |mut commands: Commands| {
                commands.spawn(Tasks::default());
            })
            .update();
    }
}
