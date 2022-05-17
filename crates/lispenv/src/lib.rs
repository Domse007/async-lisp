mod threadfun;
mod threadmessage;
mod worker;

use std::collections::HashMap;

use bidirchannel::ComServer;
use lisptype::lisptype::LispObject;
use lisptype::LispFn;
use threadmessage::{EnvToThreadMessage, ThreadToEnvMessage};
use worker::Worker;

pub enum NumWorkers {
    Minimum,
    Custom(u32),
}

impl NumWorkers {
    pub(crate) fn get_val(&self) -> u32 {
        if let Self::Custom(num) = self {
            assert!(num >= &1);
            *num
        } else {
            1
        }
    }
}

#[derive(Debug)]
pub struct LispEnvironment {
    globals: HashMap<String, LispObject>,
    functions: HashMap<String, LispFn>,
    com_server: ComServer<EnvToThreadMessage, ThreadToEnvMessage>,
    workers: Vec<Worker>,
    num_workers: u32,
}

impl LispEnvironment {
    pub fn new(num_workers: NumWorkers) -> Self {
        Self {
            globals: HashMap::new(),
            functions: HashMap::new(),
            com_server: ComServer::new(),
            workers: vec![],
            num_workers: num_workers.get_val(),
        }
    }

    pub fn add_global(&mut self, name: impl ToString, val: LispObject) {
        self.globals.insert(name.to_string(), val);
    }

    pub fn get_global(&self, name: impl ToString) -> Option<LispObject> {
        self.globals.get(&name.to_string()).cloned()
    }

    pub fn setup(&mut self) {
        for _ in 0..self.num_workers {
            self.workers.push(Worker::new(self.com_server.new_client()));
        }
    }

    pub fn run(&mut self, init_fn: impl ToString) {
        loop {
            let msg = self.com_server.recv();

            match msg.get_message() {
                ThreadToEnvMessage::GetVal(name) => {
                    self.com_server
                        .send(msg.answer(EnvToThreadMessage::ReturnRequestedGlobal(
                            self.globals.get(name.as_str()).cloned(),
                        )))
                }
                ThreadToEnvMessage::Call(fn_name) => {
                    self.com_server
                        .send(msg.answer(EnvToThreadMessage::ReturnCall(
                            self.functions.get(fn_name.as_str()).cloned(),
                        )))
                }
                _ => unimplemented!(),
            }

            let not_working = self.workers.iter().filter(|e| e.is_busy()).count();
            if not_working == 0 {
                break;
            }
        }

        self.shutdown();
    }

    pub fn shutdown(&mut self) {
        self.com_server.announce(EnvToThreadMessage::Shutdown);
        while let Some(worker) = self.workers.pop() {
            worker.shutdown();
        }
    }
}
