use bevy::{log::LogPlugin, prelude::*, time::TimeUpdateStrategy};
use bevy_bae::{plan::Plan, prelude::*};
use std::sync::Mutex;

#[test]
fn behavior_operator() {
    assert_plan(op("a"), vec!["a"]);
}

#[test]
fn sequence_single() {
    assert_plan(tasks!(Sequence[op("a")]), vec!["a"]);
}

#[test]
fn sequence_multi() {
    assert_plan(tasks!(Sequence[op("a"), op("b")]), vec!["a", "b"]);
}

#[test]
fn sequence_nested_1() {
    assert_plan(
        tasks!(Sequence[
            tasks!(Sequence[op("a"), op("b")]),
            op("c")
        ]),
        vec!["a", "b", "c"],
    );
}

#[test]
fn sequence_nested_2() {
    assert_plan(
        tasks!(Sequence[
            op("a"),
            tasks!(Sequence[op("b"), op("c")]),
        ]),
        vec!["a", "b", "c"],
    );
}

#[test]
fn sequence_nested_3() {
    assert_plan(
        tasks!(Sequence[
            tasks!(Sequence[op("a"), op("b")]),
            tasks!(Sequence[op("c"), op("d")]),
        ]),
        vec!["a", "b", "c", "d"],
    );
}

#[test]
fn select_single() {
    assert_plan(tasks!(Select[op("a")]), vec!["a"]);
}

#[test]
fn select_first() {
    assert_plan(tasks!(Select[op("a"), op("b")]), vec!["a"]);
}

#[test]
fn select_first_conditional() {
    assert_plan(
        tasks!(Select[
            (
                cond(true),
                op("a")
            ),
            op("b")
        ]),
        vec!["a"],
    );
}

#[test]
fn select_second() {
    assert_plan(
        tasks!(Select[
            (
                cond(false),
                op("a")
            ),
            op("b")
        ]),
        vec!["b"],
    );
}

#[test]
fn select_second_conditional() {
    assert_plan(
        tasks!(Select[
            (
                cond(false),
                op("a")
            ),
            (
                cond(true),
                op("b")
            ),
        ]),
        vec!["b"],
    );
}

fn assert_plan(behavior: impl Bundle, plan: Vec<&'static str>) {
    let mut app = App::new();
    let behavior = Mutex::new(Some(behavior));
    app.add_plugins((
        MinimalPlugins,
        LogPlugin {
            filter: format!(
                "bevy_log=off,bevy_bae=debug,{default}",
                default = bevy::log::DEFAULT_FILTER
            ),
            ..default()
        },
        BaePlugin::default(),
    ))
    .insert_resource(TimeUpdateStrategy::ManualDuration(
        Time::<Fixed>::default().timestep(),
    ))
    .add_systems(PreUpdate, move |mut commands: Commands| {
        commands
            .spawn(behavior.lock().unwrap().take().unwrap())
            .insert_if_new(Name::new("root"))
            .update_plan();
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

// The following functions are not reflective of real user code and are here to make the test suite more simple to set up.

fn op(name: &str) -> impl Bundle {
    (Name::new(name.to_string()), Operator::noop())
}

fn cond(val: bool) -> impl Bundle {
    conditions![if val {
        Condition::always_true()
    } else {
        Condition::always_false()
    }]
}
