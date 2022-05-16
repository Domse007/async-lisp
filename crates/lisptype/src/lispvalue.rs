use crate::lisptype::LispType;
use lisperror::LispError;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum LispValue {
    Cons(Box<(LispType, LispType)>),
    List(Vec<LispType>),
    Number(f64),
    Error(LispError),
    Symbol(String),
    Nil,
}
