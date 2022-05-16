use lisptype::lisptype::LispType;
use lisptype::LispFn;

pub enum ThreadMessage {
    // Env to Thread
    Shutdown,
    Eval(LispType),
    ReturnRequestedGlobal(Option<LispType>),
    ReturnCall(Option<LispFn>),
    // Thread to Env
    GetVal(String),
    Call(String),
    ReturnVal(LispType),
}
