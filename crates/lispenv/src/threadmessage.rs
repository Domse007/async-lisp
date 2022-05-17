use lisptype::lisptype::LispObject;
use lisptype::LispFn;

#[derive(Clone, Debug)]
pub enum EnvToThreadMessage {
    Shutdown,
    Eval(LispObject),
    ReturnRequestedGlobal(Option<LispObject>),
    ReturnCall(Option<LispFn>),
}

#[derive(Clone, Debug)]
pub enum ThreadToEnvMessage {
    GetVal(String),
    Call(String),
    ReturnVal(LispObject),
}
