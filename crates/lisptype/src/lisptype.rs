use crate::lispvalue::LispValue;
use lisperror::LispError;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LispType {
    quoted: bool,
    value: LispValue,
}

impl LispType {
    pub fn quote(mut self) -> Self {
        self.quoted = true;
        self
    }

    pub fn cons(val1: LispType, val2: LispType) -> Self {
        Self {
            quoted: false,
            value: LispValue::Cons(Box::new((val1, val2))),
        }
    }

    pub fn number(num: f64) -> Self {
        Self {
            quoted: false,
            value: LispValue::Number(num),
        }
    }

    pub fn error(err: LispError) -> Self {
        Self {
            quoted: false,
            value: LispValue::Error(err),
        }
    }

    pub fn symbol<T: ToString>(sym: T) -> Self {
        Self {
            quoted: false,
            value: LispValue::Symbol(sym.to_string()),
        }
    }

    pub fn list(list: &[LispType]) -> Self {
        Self {
            quoted: false,
            value: LispValue::List(list.into()),
        }
    }

    pub fn nil() -> Self {
        Self {
            quoted: false,
            value: LispValue::Nil,
        }
    }
}
