use crate::lispvalue::LispValue;
use lisperror::LispError;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LispObject {
    pub quoted: bool,
    pub value: LispValue,
}

impl LispObject {
    pub fn quote(mut self) -> Self {
        self.quoted = true;
        self
    }

    pub fn cons(val1: LispObject, val2: LispObject) -> Self {
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

    pub fn symbol(sym: impl ToString) -> Self {
        Self {
            quoted: false,
            value: LispValue::Symbol(sym.to_string()),
        }
    }

    pub fn list(list: &[LispObject]) -> Self {
        Self {
            quoted: false,
            value: LispValue::List(list.into()),
        }
    }

    pub fn string(str: impl ToString) -> Self {
        Self {
            quoted: false,
            value: LispValue::String(str.to_string()),
        }
    }

    pub fn nil() -> Self {
        Self {
            quoted: false,
            value: LispValue::Nil,
        }
    }
}
