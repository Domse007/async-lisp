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
