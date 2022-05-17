use crate::lisptype::LispObject;

#[derive(Clone, Debug)]
pub struct LispFn {
    params: LispObject,
    body: LispObject,
}

impl LispFn {
    pub fn new(params: LispObject, body: LispObject) -> Self {
        Self { params, body }
    }
}
