use alloc::collections::VecDeque;
use bevy_derive::DerefMut;

use crate::{prelude::*, task::primitive::OperatorId};

pub mod execution;
pub mod update;

#[derive(Component, Clone, Default, Reflect, Debug, Deref, DerefMut)]
#[reflect(Component)]
pub struct Plan(#[reflect(ignore)] pub VecDeque<PlannedOperator>);

#[derive(Clone, Debug)]
pub struct PlannedOperator {
    pub system: OperatorId,
    pub entity: Entity,
    pub effects: Vec<Effect>,
}
