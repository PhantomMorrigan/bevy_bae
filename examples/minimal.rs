use bevy::prelude::*;
use bevy_bae::prelude::*;

fn main() {}

fn trunk_thumper_domain() -> impl Bundle {
    (
        Name::new("Be Trunk Thumper"),
        tasks!(Select[
                (
                    Name::new("Beat up enemy"),
                    conditions![(
                        Name::new("Can see enemy"),
                        ObserverCondition(|check: On<CheckCondition>, spatial_query: SpatialQuery| {
                            spatial_query.cast_ray(...).is_some()
                        }),
                    )],
                    tasks!(Sequence[
                        (
                            Name::new("Navigate to enemy"),
                            tasks!(Step::new(navigate_to_enemy)),
                            effects![PropEffect("location", "enemy")],
                        ),
                        (
                            Name::new("Do trunk slam"),
                            tasks!(Step::new(do_trunk_slam)),
                            effects![PropEffect("play_sound", "Aha! Take this!!!")],
                        ),
                    ]),
                ),
                (
                    Name::new("Patrol bridges"),
                    conditions![AlwaysTrue],
                    tasks!(Sequence[
                        (
                            Name::new("Choose best bridge to check for enemies"),
                            tasks!(Step::new(choose_bridge_to_check)),
                            effects![PropEffect("play_sound", "Hmm where are those disgusting humies..")],
                        ),
                        (
                            Name::new("Go to bridge"),
                            tasks!(Step::new(navigate_to_bridge)),
                            effects![PropEffect("location", "bridge")],
                        ),
                        (
                            Name::new("Check if anything is out of the ordinary"),
                            tasks!(Step::new(check_bridge)),
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
