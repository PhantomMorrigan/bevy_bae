use crate::prelude::*;

pub mod relationship;

#[derive(Component, Reflect, Debug, PartialEq, Clone, Copy)]
#[reflect(Component)]
pub struct Effect;
