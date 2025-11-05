use alloc::slice;
use bevy_ecs::relationship::{RelatedSpawner, RelatedSpawnerCommands};
use core::iter::Copied;

use crate::prelude::*;

#[derive(Component, Deref, Reflect, Debug, PartialEq, Eq, Clone)]
#[relationship(relationship_target = Effects)]
#[reflect(Component)]
pub struct EffectOf(pub Entity);

#[derive(Component, Deref, Reflect, Debug, Default, PartialEq, Eq)]
#[relationship_target(relationship = EffectOf, linked_spawn)]
pub struct Effects(Vec<Entity>);

impl<'a> IntoIterator for &'a Effects {
    type Item = Entity;
    type IntoIter = Copied<slice::Iter<'a, Entity>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub type EffectSpawner<'w> = RelatedSpawner<'w, EffectOf>;

pub type EffectSpawnerCommands<'w> = RelatedSpawnerCommands<'w, EffectOf>;

#[macro_export]
macro_rules! effects {
    [$($effect:expr),*$(,)?] => {
        ::bevy::prelude::related!($crate::prelude::Effects[$($crate::prelude::IntoEffectBundle::into_effect_bundle($effect)),*])
    };
}

pub use effects;

pub trait IntoEffectBundle {
    /// Returns a bundle for a binding.
    fn into_effect_bundle(self) -> impl Bundle;
}

impl<B: Into<Effect>> IntoEffectBundle for B {
    fn into_effect_bundle(self) -> impl Bundle {
        self.into()
    }
}

macro_rules! impl_into_binding_bundle {
    ($($C:ident),*) => {
        impl<B: Into<Effect>, $($C: Bundle,)*> IntoEffectBundle for (B, $($C),*) {
            #[allow(non_snake_case, reason = "tuple unpack")]
            fn into_effect_bundle(self) -> impl Bundle {
                let (b, $($C),* ) = self;
                (b.into(), $($C),*)
            }
        }
    }
}

variadics_please::all_tuples!(impl_into_binding_bundle, 0, 14, C);
