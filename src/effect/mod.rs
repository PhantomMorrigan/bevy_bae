use ustr::Ustr;

use crate::prelude::*;

pub mod relationship;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Effect {
    #[reflect(ignore, default = "Effect::noop")]
    effect: Box<dyn FnMut(&mut Props) + Send + Sync + 'static>,
}

impl Effect {
    pub fn new(predicate: impl FnMut(&mut Props) + Send + Sync + 'static) -> Self {
        Self {
            effect: Box::new(predicate),
        }
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

    fn noop() -> Box<dyn FnMut(&mut Props) + Send + Sync + 'static> {
        Box::new(|_| {})
    }
}
