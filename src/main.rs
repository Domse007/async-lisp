use lispenv::{LispEnvironment, NumWorkers};
use lisperror::LispError;
use lisptype::lisptype::LispType;

fn main() {
    let error = LispError::invalid_type("hello", "world").msg("This is a test.");
    let lisp = LispType::number(32.).quote();

    let mut env = LispEnvironment::new(NumWorkers::new(1));

    env.run("main");

    println!("{}", error);
    println!("{:?}", lisp);
}
