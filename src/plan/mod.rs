use alloc::collections::VecDeque;
use bevy_derive::DerefMut;

use crate::{plan::mtr::Mtr, prelude::*, task::primitive::OperatorId};

pub mod execution;
pub mod mtr;
pub mod update;

#[derive(Component, Clone, Default, PartialEq, Eq, Reflect, Debug, Deref, DerefMut)]
#[reflect(Component)]
#[require(Props)]
pub struct Plan {
    #[reflect(ignore)]
    #[deref]
    pub operators: VecDeque<PlannedOperator>,
    pub full_entities: Vec<Entity>,
    pub mtr: Mtr,
}

impl Plan {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlannedOperator {
    pub system: OperatorId,
    pub entity: Entity,
    pub effects: Vec<Effect>,
}
