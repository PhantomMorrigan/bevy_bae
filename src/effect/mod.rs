use crate::prelude::*;
use core::fmt::Debug;
use ustr::Ustr;

pub mod relationship;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Effect {
    #[reflect(ignore, default = "Effect::noop")]
    effect: Box<dyn Fn(&mut Props) + Send + Sync + 'static>,
}

impl Effect {
    pub fn new(predicate: impl Fn(&mut Props) + Send + Sync + 'static) -> Self {
        Self {
            effect: Box::new(predicate),
        }
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

        Self::new(move |props| props.set(name, !props.get_value(name).unwrap().bool()))
    }

    pub fn inc<T: Into<Value> + Default>(
        name: impl Into<Ustr>,
        value: impl Into<Value> + Default,
    ) -> Self {
        Self::mutate(name, value, |a, b| Value::Num(a.num() + b.num()))
    }

    pub fn dec<T: Into<Value> + Default>(
        name: impl Into<Ustr>,
        value: impl Into<Value> + Default,
    ) -> Self {
        Self::mutate(name, value, |a, b| Value::Num(a.num() - b.num()))
    }

    pub fn mul(name: impl Into<Ustr>, value: impl Into<Value> + Default) -> Self {
        Self::mutate(name, value, |a, b| Value::Num(a.num() * b.num()))
    }

    pub fn div(name: impl Into<Ustr>, value: impl Into<Value> + Default) -> Self {
        Self::mutate(name, value, |a, b| Value::Num(a.num() / b.num()))
    }

    pub fn pow(name: impl Into<Ustr>, value: impl Into<Value> + Default) -> Self {
        Self::mutate(name, value, |a, b| Value::Num(a.num().powf(b.num())))
    }

    pub fn rem(name: impl Into<Ustr>, value: impl Into<Value> + Default) -> Self {
        Self::mutate(name, value, |a, b| Value::Num(a.num() % b.num()))
    }

    pub fn mutate<T: Into<Value> + Default>(
        name: impl Into<Ustr>,
        value: T,
        mutate: impl Fn(Value, Value) -> Value + Send + Sync + 'static,
    ) -> Self {
        let name = name.into();
        let value = value.into();
        Self::new(move |props| {
            if props.get_value(name).is_none() {
                props.set(name, T::default())
            };
            props.set(name, mutate(props.get_value(name).unwrap(), value));
        })
    }

    fn noop() -> Box<dyn Fn(&mut Props) + Send + Sync + 'static> {
        Box::new(|_| {})
    }
}

impl Debug for Effect {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Effect")
            .field("effect", &"<callback>")
            .finish()
    }
}
