use bevy::prelude::*;
use bevy_bae::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin, BaePlugin::default()))
        .add_systems(Startup, setup)
        .add_observer(update_state)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Plan::new(),
        Select,
        tasks![
            (
                Operator::new(greet),
                conditions![Condition::eq("greet_mode", "on")]
            ),
            (
                Operator::new(say_stop),
                conditions![Condition::eq("greet_mode", "ending")],
                effects![Effect::set("greet_mode", false)]
            ),
            Operator::new(idle),
        ],
    ));
}

fn greet(_: In<OperatorInput>) -> TaskStatus {
    info!("Oh hai!!! (Press right mouse button to stop the spam.)");
    TaskStatus::Success
}

fn say_stop(_: In<OperatorInput>) -> TaskStatus {
    info!("Ok ok I will stop now :<");
    TaskStatus::Success
}

fn idle(_: In<OperatorInput>) -> TaskStatus {
    // nothing to do
    TaskStatus::Success
}

fn update_state(press: On<Pointer<Press>>, mut props: Single<&mut Props, With<Plan>>) {
    match press.button {
        PointerButton::Primary => props.set("greet_mode", "on"),
        PointerButton::Secondary => {
            if props.get::<Ustr>("greet_mode") == "on" {
                props.set("greet_mode", "ending")
            }
        }
        _ => (),
    }
}
