use std::sync::mpsc::{channel, Receiver, RecvError, Sender, TryRecvError};

pub struct BiDirChannel<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

impl<T: Send> BiDirChannel<T> {
    pub fn new() -> (Self, Self) {
        let (snd1, rcv1) = channel();
        let (snd2, rcv2) = channel();

        let c1 = Self {
            sender: snd1,
            receiver: rcv2,
        };

        let c2 = Self {
            sender: snd2,
            receiver: rcv1,
        };

        (c1, c2)
    }

    pub fn recv(&self) -> T {
        self.receiver.recv().unwrap()
    }

    pub fn try_recv(&self) -> Option<T> {
        match self.receiver.try_recv() {
            Ok(val) => Some(val),
            Err(e) => match e {
                TryRecvError::Empty => None,
                TryRecvError::Disconnected => unreachable!(),
            },
        }
    }

    pub fn send(&self, msg: T) {
        self.sender.send(msg).unwrap();
    }
}
