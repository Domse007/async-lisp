use crate::lisptype::LispType;

#[derive(Clone, Debug)]
pub struct LispFn {
    params: LispType,
    body: LispType,
}

impl LispFn {
    pub fn new(params: LispType, body: LispType) -> Self {
        Self { params, body }
    }
}
