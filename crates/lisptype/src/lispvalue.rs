use crate::lisptype::LispObject;
use lisperror::LispError;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum LispValue {
    Cons(Box<(LispObject, LispObject)>),
    List(Vec<LispObject>),
    Number(f64),
    Error(LispError),
    Symbol(String),
    String(String),
    Nil,
}

impl LispValue {
    pub fn get_type_str(&self) -> &'static str {
        match self {
            Self::Cons(_) => "cons",
            Self::List(_) => "list",
            Self::Number(_) => "number",
            Self::Error(_) => "error",
            Self::Symbol(_) => "symbol",
            Self::String(_) => "string",
            Self::Nil => "nil",
        }
    }
}
