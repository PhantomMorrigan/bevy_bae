use core::fmt::Debug;
use core::ops::RangeBounds;

use ustr::Ustr;

use crate::prelude::*;

pub mod relationship;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Condition {
    #[reflect(ignore, default = "Condition::true_pred")]
    predicate: Box<dyn Fn(&Props) -> bool + Send + Sync + 'static>,
}

impl Condition {
    pub fn new(predicate: impl Fn(&Props) -> bool + Send + Sync + 'static) -> Self {
        Self {
            predicate: Box::new(predicate),
        }
    }

    pub fn is_fullfilled(&self, props: &Props) -> bool {
        (self.predicate)(props)
    }

    pub fn eq(name: impl Into<Ustr>, value: impl Into<Value>) -> Self {
        Self::predicate(name, value, |a, b| a.eq(b))
    }

    pub fn ne(name: impl Into<Ustr>, value: impl Into<Value>) -> Self {
        Self::predicate(name, value, |a, b| a.ne(b))
    }

    pub fn gt(name: impl Into<Ustr>, value: impl Into<Value>) -> Self {
        Self::predicate(name, value, |a, b| a.num() > b.num())
    }

    pub fn gte(name: impl Into<Ustr>, value: impl Into<Value>) -> Self {
        Self::predicate(name, value, |a, b| a.num() >= b.num())
    }

    pub fn lt(name: impl Into<Ustr>, value: impl Into<Value>) -> Self {
        Self::predicate(name, value, |a, b| a.num() < b.num())
    }

    pub fn lte(name: impl Into<Ustr>, value: impl Into<Value>) -> Self {
        Self::predicate(name, value, |a, b| a.num() <= b.num())
    }

    pub fn in_range(
        name: impl Into<Ustr>,
        range: impl RangeBounds<f32> + Send + Sync + 'static,
    ) -> Self {
        let name = name.into();
        Self::new(move |props| {
            let Some(prop) = props.get_value(name) else {
                return false;
            };
            range.contains(&prop.num())
        })
    }

    pub fn always_true() -> Self {
        Self::new(|_| true)
    }

    pub fn always_false() -> Self {
        Self::new(|_| false)
    }

    pub fn predicate(
        name: impl Into<Ustr>,
        value: impl Into<Value>,
        predicate: impl Fn(Value, Value) -> bool + Send + Sync + 'static,
    ) -> Self {
        let name = name.into();
        let value = value.into();
        Self::new(move |props| {
            let Some(prop) = props.get_value(name) else {
                return false;
            };
            predicate(prop, value)
        })
    }

    fn true_pred() -> Box<dyn Fn(&Props) -> bool + Send + Sync + 'static> {
        Box::new(|_| true)
    }
}

impl Debug for Condition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Condition")
            .field("predicate", &"<callback>")
            .finish()
    }
}
