use crate::prelude::*;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Sequence;

impl CompoundTask for Sequence {}
