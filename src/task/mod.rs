//! Types for dealing with [`tasks!`].

use crate::prelude::*;

pub mod compound;
pub mod operator;
pub(crate) mod validation;

/// The return type of [`Operator`]s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum OperatorStatus {
    /// The task has completed successfully. Proceed to the next step of the plan.
    Success,
    /// The task is still running. Stay in the current step of the plan.
    Ongoing,
    /// The task has failed. Abort the plan and replan it at the next fixed frame.
    Failure,
}
