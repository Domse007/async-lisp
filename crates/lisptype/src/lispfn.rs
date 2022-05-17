use std::collections::HashMap;

use lisperror::LispError;

use crate::lisptype::LispObject;
use crate::lispvalue::LispValue;

#[derive(Clone, Debug)]
pub struct LispFn {
    fn_name: String,
    params: LispObject,
    body: LispObject,
    var_scope: HashMap<String, LispObject>,
}

impl LispFn {
    pub fn new(fn_name: String, params: LispObject, body: LispObject) -> Self {
        Self {
            fn_name,
            params,
            body,
            var_scope: HashMap::new(),
        }
    }

    pub fn build_args(&mut self, args: LispObject) -> LispObject {
        let type_str = args.value.get_type_str();
        match (args.value, self.params.value.clone()) {
            (LispValue::List(list), LispValue::List(names)) => {
                if list.len() != names.len() {
                    return LispObject::error(LispError::wrong_number_of_args(
                        self.fn_name.to_string(),
                        list.len(),
                        names.len(),
                    ));
                }

                for (key, value) in names.iter().zip(list) {
                    if let LispValue::Symbol(sym) = &key.value {
                        self.var_scope.insert(sym.to_string(), value)
                    } else {
                        return LispObject::error(LispError::invalid_type(
                            key.value.get_type_str(),
                            value.value.get_type_str(),
                        ));
                    };
                }
            }
            _ => return LispObject::error(LispError::invalid_type("list", type_str)),
        }
        LispObject::nil()
    }

    pub fn run(&mut self, args: LispObject) -> LispObject {
        if let LispValue::Error(err) = self.build_args(args).value {
            return LispObject::error(err);
        }

        for body_element in self.body {
            // Eval the body. Most likely requires self.var_scope to be passed in.
            todo!();
        }

        LispObject::nil()
    }
}
