use bevy::{prelude::*, time::TimeUpdateStrategy};
use bevy_bae::{plan::Plan, prelude::*};

#[test]
fn single_operator() {
    assert_plan(|| operator("a"), vec!["a"]);
}

#[test]
fn single_sequence() {
    assert_plan(
        || (Name::new("a"), tasks!(Sequence[operator("a")])),
        vec!["a"],
    );
}

#[test]
fn single_select() {
    assert_plan(
        || (Name::new("a"), tasks!(Select[operator("a")])),
        vec!["a"],
    );
}

fn assert_plan<T, U>(behavior: T, plan: Vec<&'static str>)
where
    T: Fn() -> U + Send + Sync + 'static,
    U: Bundle,
{
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, BaePlugin::default()))
        .insert_resource(TimeUpdateStrategy::ManualDuration(
            Time::<Fixed>::default().timestep(),
        ))
        .add_systems(PreUpdate, move |mut commands: Commands| {
            commands.spawn(behavior()).update_plan();
        });
    app.finish();
    app.update();
    let actual_plan = app
        .world()
        .try_query::<&Plan>()
        .unwrap()
        .single(app.world())
        .unwrap()
        .clone();

    let mut operators = app.world().try_query::<(&Operator, &Name)>().unwrap();
    let actual_plan_names = actual_plan
        .0
        .into_iter()
        .map(|op_to_search| {
            operators
                .iter(app.world())
                .find_map(|(op, name)| (op.system_id() == op_to_search).then(|| name.to_string()))
                .unwrap()
        })
        .collect::<Vec<_>>();

    let plan_names = plan.into_iter().map(|n| n.to_string()).collect::<Vec<_>>();

    assert_eq!(plan_names, actual_plan_names);
}

fn operator(name: &str) -> impl Bundle {
    (Name::new(name.to_string()), Operator::noop())
}
