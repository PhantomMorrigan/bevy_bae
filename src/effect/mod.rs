use crate::prelude::*;
use alloc::sync::Arc;
use core::fmt::Debug;
use ustr::Ustr;

pub mod relationship;

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
pub struct Effect {
    #[reflect(ignore, default = "Effect::noop")]
    effect: Arc<dyn Fn(&mut Props) + Send + Sync + 'static>,
    pub plan_only: bool,
}

impl Effect {
    pub fn new(predicate: impl Fn(&mut Props) + Send + Sync + 'static) -> Self {
        Self {
            effect: Arc::new(predicate),
            plan_only: false,
        }
    }

    pub fn plan_only(mut self) -> Self {
        self.plan_only = true;
        self
    }

    pub fn apply(&self, props: &mut Props) {
        (self.effect)(props);
    }

    pub fn set(name: impl Into<Ustr>, value: impl Into<Value>) -> Self {
        let name = name.into();
        let value = value.into();
        Self::new(move |props| props.set(name, value))
    }

    pub fn toggle(name: impl Into<Ustr>) -> Self {
        let name = name.into();

        Self::new(move |props| {
            let val = props.get_mut::<bool>(name);
            *val = !*val;
        })
    }

    pub fn inc<T: Into<Value>>(name: impl Into<Ustr>, value: impl Into<Value>) -> Self {
        Self::mutate(name, value, |a, b| *a += b)
    }

    pub fn dec<T: Into<Value> + Default>(
        name: impl Into<Ustr>,
        value: impl Into<Value> + Default,
    ) -> Self {
        Self::mutate(name, value, |a, b| *a -= b)
    }

    pub fn mul(name: impl Into<Ustr>, value: impl Into<Value> + Default) -> Self {
        Self::mutate(name, value, |a, b| *a *= b)
    }

    pub fn div(name: impl Into<Ustr>, value: impl Into<Value> + Default) -> Self {
        Self::mutate(name, value, |a, b| *a /= b)
    }

    pub fn mutate(
        name: impl Into<Ustr>,
        value: impl Into<Value>,
        mutate: impl Fn(&mut Value, Value) + Send + Sync + 'static,
    ) -> Self {
        let name = name.into();
        let value = value.into();
        Self::new(move |props| {
            let prop = props.entry(name).or_default();
            mutate(prop, value);
        })
    }

    fn noop() -> Arc<dyn Fn(&mut Props) + Send + Sync + 'static> {
        Arc::new(|_| {})
    }
}

impl Debug for Effect {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Effect")
            .field("effect", &"<callback>")
            .finish()
    }
}
