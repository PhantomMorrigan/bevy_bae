//! Shows a minimal example of using BAE.
//! The NPC will greet the player once per second.

use bevy::prelude::*;
use bevy_bae::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin, BaePlugin::default()))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Plan::new(),
        Sequence,
        tasks![Operator::new(greet), Operator::new(idle),],
    ));
}

fn greet(_: In<OperatorInput>) -> OperatorStatus {
    info!("Oh hai!!! I greet you every second. Very polite, eh?");
    OperatorStatus::Success
}

fn idle(_: In<OperatorInput>, time: Res<Time>, mut timer: Local<Option<Timer>>) -> OperatorStatus {
    let timer = timer.get_or_insert_with(|| Timer::from_seconds(1.0, TimerMode::Once));
    timer.tick(time.delta());
    if timer.is_finished() {
        timer.reset();
        OperatorStatus::Success
    } else {
        OperatorStatus::Ongoing
    }
}

use bevy::prelude::*;
use bevy_bae::prelude::*;

fn spawn_npc(mut commands: Commands) {
    commands.spawn((
        Plan::new(),
        Sequence,
        tasks![
            (
                Operator::new(prepare_to_greet),
                effects![Effect::set("can_greet", true)],
            ),
            (
                conditions![Condition::eq("can_greet", true)],
                Operator::new(greet)
            ),
        ],
    ));
}
fn prepare_to_greet(In(_input): In<OperatorInput>) -> OperatorStatus {
    OperatorStatus::Success
}

fn greet(In(_input): In<OperatorInput>) -> OperatorStatus {
    info!("Oh hai!");
    OperatorStatus::Success
}
