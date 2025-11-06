use bevy::prelude::*;
use bevy_bae::prelude::*;

fn main() {}

fn trunk_thumper_domain() -> impl Bundle {
    (
        Name::new("Be Trunk Thumper"),
        Select,
        tasks!(Select[
                (
                    Name::new("Beat up enemy"),
                    conditions![(
                        Condition::new(|In(entity): In<Entity>, names: Query<NameOrEntity>| -> bool {
                            true
                        }),
                    )],
                    Sequence,
                    tasks!(Sequence[
                        (
                            Name::new("Navigate to enemy"),
                            Step::new(navigate_to_enemy),
                            effects![PropEffect("location", "enemy")],
                        ),
                        (
                            Name::new("Do trunk slam"),
                            Step::new(do_trunk_slam),
                            effects![PropEffect("play_sound", "Aha! Take this!!!")],
                        ),
                    ]),
                ),
                (
                    Name::new("Patrol bridges"),
                    conditions![AlwaysTrue],
                    Sequence,
                    tasks!(Sequence[
                        (
                            Name::new("Choose best bridge to check for enemies"),
                            Step::new(choose_bridge_to_check),
                            effects![PropEffect("play_sound", "Hmm where are those disgusting humies..")],
                        ),
                        (
                            Name::new("Go to bridge"),
                            Step::new(navigate_to_bridge),
                            effects![PropEffect("location", "bridge")],
                        ),
                        (
                            Name::new("Check if anything is out of the ordinary"),
                            Step::new(check_bridge),
                            effects![PropEffect("play_sound", "Alright, let's see what we have here.")],
                        ),
                    ]),
                )
            ]
        ),
    )
}

fn navigate_to_enemy(_step: On<ExecuteStep>) -> TaskStatus {
    info!("navigating to enemy");
    TaskStatus::Success
}

fn do_trunk_slam(_step: On<ExecuteStep>) -> TaskStatus {
    info!("trunk slam");
    TaskStatus::Success
}

fn choose_bridge_to_check(_step: On<ExecuteStep>) -> TaskStatus {
    info!("choosing bridge to check");
    TaskStatus::Success
}

fn navigate_to_bridge(_step: On<ExecuteStep>) -> TaskStatus {
    info!("navigating to bridge");
    TaskStatus::Success
}

fn check_bridge(_step: On<ExecuteStep>) -> TaskStatus {
    info!("checking bridge");
    TaskStatus::Success
}
