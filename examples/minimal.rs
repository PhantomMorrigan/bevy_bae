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
                    conditions![
                        (Condition::eq("can_see_enemy", true), Name::new("Can see enemy"))
                    ],
                    Sequence,
                    tasks!(Sequence[
                        (
                            Name::new("Navigate to enemy"),
                            TaskSystem::new(navigate_to_enemy),
                            effects![
                               (Effect::set("location", "enemy"), Name::new("Set location to enemy position"))
                            ],
                        ),
                        (
                            Name::new("Do trunk slam"),
                            TaskSystem::new(do_trunk_slam),
                        ),
                    ]),
                ),
                (
                    Name::new("Patrol bridges"),
                    conditions![(Condition::always_true(), Name::new("Always true"))],
                    Sequence,
                    tasks!(Sequence[
                        (
                            Name::new("Choose best bridge to check for enemies"),
                            TaskSystem::new(choose_bridge_to_check),
                        ),
                        (
                            Name::new("Go to bridge"),
                            TaskSystem::new(navigate_to_bridge),
                            effects![(Effect::set("location", "bridge"), Name::new("Set location to bridge"))],
                        ),
                        (
                            Name::new("Check if anything is out of the ordinary"),
                            TaskSystem::new(check_bridge),
                        ),
                    ]),
                )
            ]
        ),
    )
}

fn navigate_to_enemy(_step: In<Entity>) -> TaskStatus {
    info!("navigating to enemy");
    TaskStatus::Success
}

fn do_trunk_slam(_step: In<Entity>) -> TaskStatus {
    info!("trunk slam");
    TaskStatus::Success
}

fn choose_bridge_to_check(_step: In<Entity>) -> TaskStatus {
    info!("choosing bridge to check");
    TaskStatus::Success
}

fn navigate_to_bridge(_step: In<Entity>) -> TaskStatus {
    info!("navigating to bridge");
    TaskStatus::Success
}

fn check_bridge(_step: In<Entity>) -> TaskStatus {
    info!("checking bridge");
    TaskStatus::Success
}
