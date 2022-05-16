use std::thread::{self, JoinHandle};

use bidirchannel::BiDirChannel;

use crate::threadfun;
use crate::threadmessage::ThreadMessage;

pub struct Worker {
    handle: JoinHandle<()>,
    channel: BiDirChannel<ThreadMessage>,
    working: bool,
}

impl Worker {
    pub fn new() -> Self {
        let (endp1, endp2) = BiDirChannel::new();

        Self {
            handle: thread::spawn(move || threadfun::run(endp1)),
            channel: endp2,
            working: false,
        }
    }

    pub fn send(&self, msg: ThreadMessage) -> Option<()> {
        if !self.working {
            self.channel.send(msg);
            Some(())
        } else {
            None
        }
    }

    pub fn shutdown(self) {
        self.channel.send(ThreadMessage::Shutdown);
        self.handle.join().unwrap();
    }

    pub fn try_recv(&self) -> Option<ThreadMessage> {
        self.channel.try_recv()
    }

    pub fn is_busy(&self) -> bool {
        self.working
    }
}
