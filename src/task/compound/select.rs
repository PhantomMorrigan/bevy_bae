use crate::prelude::*;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Select;

impl CompoundTask for Select {}
