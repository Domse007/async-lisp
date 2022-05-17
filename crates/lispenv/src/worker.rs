use std::thread::{self, JoinHandle};

use bidirchannel::ComClient;

use crate::threadfun;
use crate::threadmessage::{EnvToThreadMessage, ThreadToEnvMessage};

pub struct Worker {
    handle: JoinHandle<()>,
    working: bool,
}

impl Worker {
    pub fn new(com_adaptor: ComClient<EnvToThreadMessage, ThreadToEnvMessage>) -> Self {
        Self {
            handle: thread::spawn(move || threadfun::run(com_adaptor)),
            working: false,
        }
    }

    pub fn is_busy(&self) -> bool {
        self.working
    }

    pub fn shutdown(self) {
        self.handle.join().unwrap();
    }
}
