use std::env;

use lispenv::{LispEnvironment, NumWorkers};
use lisptype::lisptype::LispObject;

fn main() {
    let args = env::args()
        .skip(1)
        .map(LispObject::string)
        .collect::<Vec<LispObject>>();

    let mut env = LispEnvironment::new(NumWorkers::Custom(2));
    env.add_global("args", LispObject::list(&args));

    env.setup();

    env.run("main");

    env.shutdown();
}
