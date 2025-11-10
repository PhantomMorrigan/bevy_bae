use bevy::{log::LogPlugin, prelude::*, time::TimeUpdateStrategy};
use bevy_bae::prelude::*;
use std::sync::Mutex;

#[test]
fn run_plan() {
    let mut app = App::test(op("a"));
    // plan
    app.assert_last_opt(None);
    app.update();
    // run
    app.assert_last_opt(Some("a"));
}

#[test]
fn run_plans_replans_reruns() {
    let mut app = App::test(tasks!(
        Select[
            (op("a"), cond_is("use_a", true), eff("use_a", false)),
            op("b")
        ]
    ));
    // plan
    app.assert_last_opt(None);
    app.update();
    // run
    app.assert_last_opt(Some("a"));
    // replan
    app.update();
    app.assert_last_opt(Some("b"));
}

trait TestApp {
    fn test(behavior: impl Bundle) -> App;
    #[track_caller]
    fn assert_last_opt(&self, name: impl Into<Option<&'static str>>);
}

impl TestApp for App {
    fn test(behavior: impl Bundle) -> App {
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
        .init_resource::<LastOpt>()
        .add_systems(Startup, move |mut commands: Commands| {
            commands
                .spawn(behavior.lock().unwrap().take().unwrap())
                .insert_if_new(Name::new("root"))
                .trigger(UpdatePlan::new);
        })
        .add_systems(PreUpdate, |mut last_opt: ResMut<LastOpt>| {
            last_opt.0 = None;
        });
        app.finish();
        app.update();
        app
    }

    #[track_caller]
    fn assert_last_opt(&self, name: impl Into<Option<&'static str>>) {
        let name: Option<&'static str> = name.into();
        let name: Option<String> = name.map(|s| s.into());
        let actual = self.world().resource::<LastOpt>().0.clone();
        assert_eq!(actual, name);
    }
}
// The following functions are not reflective of real user code and are here to make the test suite more simple to set up.

#[derive(Resource, Default)]
struct LastOpt(Option<String>);

fn op(name: &str) -> impl Bundle {
    let name = name.to_string();
    (
        Name::new(name.clone()),
        Operator::new(
            move |_: In<OperatorInput>, mut last_opt: ResMut<LastOpt>| -> TaskStatus {
                last_opt.0 = Some(name.clone());
                TaskStatus::Success
            },
        ),
    )
}

fn cond(val: bool) -> impl Bundle {
    conditions![if val {
        Condition::always_true()
    } else {
        Condition::always_false()
    }]
}

fn cond_is(name: &str, val: impl Into<Value>) -> impl Bundle {
    conditions![Condition::eq(name, val)]
}

fn eff(name: &str, val: impl Into<Value>) -> impl Bundle {
    effects![Effect::set(name, val)]
}
