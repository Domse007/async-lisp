use std::env;

use lispenv::{LispEnvironment, NumWorkers};
use lisperror::LispError;
use lisptype::lisptype::LispObject;

fn main() {
    let args = env::args()
        .skip(1)
        .map(LispObject::string)
        .collect::<Vec<LispObject>>();
    let error = LispError::invalid_type("hello", "world").msg("This is a test.");
    let lisp = LispObject::number(32.).quote();

    let mut env = LispEnvironment::new(NumWorkers::new(2));
    env.add_global("args", LispObject::list(&args));

    dbg!(&env);

    env.run("main");

    println!("{}", error);
    println!("{:?}", lisp);
}
