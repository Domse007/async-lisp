mod threadfun;
mod threadmessage;
mod worker;

use std::collections::HashMap;
use std::thread::{self, JoinHandle};

use lisptype::lisptype::LispType;
use lisptype::LispFn;
use threadmessage::ThreadMessage;
use worker::Worker;

pub struct NumWorkers {
    pub(crate) num: u32,
}

impl NumWorkers {
    pub fn min() -> Self {
        Self { num: 1 }
    }
    pub fn new(num: u32) -> Self {
        assert!(num >= 1);
        Self { num }
    }
}

pub struct LispEnvironment {
    globals: HashMap<String, LispType>,
    functions: HashMap<String, LispFn>,
    workers: Vec<Worker>,
    num_workers: u32,
}

impl LispEnvironment {
    pub fn new(num_workers: NumWorkers) -> Self {
        Self {
            globals: HashMap::new(),
            functions: HashMap::new(),
            workers: vec![],
            num_workers: num_workers.num,
        }
    }

    pub fn add_global<T: ToString>(&mut self, name: T, val: LispType) {
        self.globals.insert(name.to_string(), val);
    }

    pub fn get_global<T: ToString>(&self, name: T) -> Option<LispType> {
        self.globals.get(&name.to_string()).cloned()
    }

    pub fn run<T: ToString>(&mut self, init_fn: T) {
        for _ in 0..self.num_workers {
            self.workers.push(Worker::new());
        }

        loop {
            for worker in self.workers.iter() {
                match worker.try_recv() {
                    Some(instr) => match instr {
                        ThreadMessage::GetVal(name) => worker
                            .send(ThreadMessage::ReturnRequestedGlobal(
                                self.globals.get(name.as_str()).cloned(),
                            ))
                            .unwrap(), // can safely unwrap here, becuase the thread is waiting.
                        ThreadMessage::Call(fn_name) => worker
                            .send(ThreadMessage::ReturnCall(
                                self.functions.get(fn_name.as_str()).cloned(),
                            ))
                            .unwrap(),
                        _ => unimplemented!(),
                    },
                    None => {}
                }
            }

            let not_working = self.workers.iter().filter(|e| e.is_busy()).count();
            if not_working == 0 {
                break;
            }
        }

        while let Some(worker) = self.workers.pop() {
            worker.shutdown();
        }
    }
}
