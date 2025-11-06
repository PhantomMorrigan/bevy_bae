use crate::prelude::*;

pub(crate) trait ValueExt: Copy {
    fn num(self) -> f32;
    fn bool(self) -> bool;
    fn str(self) -> String;
    fn eq(self, other: Self) -> bool;
    fn ne(self, other: Self) -> bool;
}

impl ValueExt for Value {
    fn num(self) -> f32 {
        self.try_into().expect("Cannot convert to number")
    }
    fn bool(self) -> bool {
        self.try_into().expect("Cannot convert to boolean")
    }
    fn str(self) -> String {
        self.to_string()
    }
    fn eq(self, other: Self) -> bool {
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Str(a), Value::Str(b)) => a == b,
            _ => panic!("Cannot compare different types"),
        }
    }
    fn ne(self, other: Self) -> bool {
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => a != b,
            (Value::Bool(a), Value::Bool(b)) => a != b,
            (Value::Str(a), Value::Str(b)) => a != b,
            _ => panic!("Cannot compare different types"),
        }
    }
}
