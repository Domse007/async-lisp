mod messages;

use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};

use messages::{ClientMessage, ServerMessage};

#[derive(Debug)]
pub struct ComServer<T, U> {
    thread_index: u32,
    senders: HashMap<u32, Sender<T>>,
    receiver: Receiver<(u32, U)>,
    backup: Sender<(u32, U)>,
}

impl<T: Sync + Clone, U: Sync + Clone> ComServer<T, U> {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            thread_index: 0,
            senders: HashMap::new(),
            receiver,
            backup: sender,
        }
    }

    pub fn new_client(&mut self) -> ComClient<T, U> {
        let (sender, receiver) = mpsc::channel();
        self.thread_index += 1;
        self.senders.insert(self.thread_index, sender);
        ComClient {
            thread_index: self.thread_index,
            sender: self.backup.clone(),
            receiver,
        }
    }

    pub fn recv(&self) -> ClientMessage<U> {
        let incomming = self.receiver.recv().unwrap();
        ClientMessage {
            thread_index: incomming.0,
            message: incomming.1,
        }
    }

    pub fn send(&self, msg: ServerMessage<T>) {
        let sender = self.senders.get(&msg.thread_index).unwrap();
        sender.send(msg.answer).unwrap();
    }

    pub fn announce(&self, msg: T) {
        self.senders
            .iter()
            .for_each(|(_, client)| client.send(msg.clone()).unwrap())
    }
}

pub struct ComClient<T, U> {
    thread_index: u32,
    sender: Sender<(u32, U)>,
    receiver: Receiver<T>,
}

impl<T, U> ComClient<T, U> {
    pub fn recv(&self) -> T {
        self.receiver.recv().unwrap()
    }

    pub fn send(&self, msg: U) {
        self.sender.send((self.thread_index, msg));
    }
}
