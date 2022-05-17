use lisptype::lisptype::LispType;
use lisptype::LispFn;

#[derive(Clone)]
pub enum EnvToThreadMessage {
    Shutdown,
    Eval(LispType),
    ReturnRequestedGlobal(Option<LispType>),
    ReturnCall(Option<LispFn>),
}

#[derive(Clone)]
pub enum ThreadToEnvMessage {
    GetVal(String),
    Call(String),
    ReturnVal(LispType),
}
