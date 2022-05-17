pub struct ClientMessage<U> {
    pub(crate) thread_index: u32,
    pub(crate) message: U,
}

impl<U: Clone> ClientMessage<U> {
    pub fn get_message(&self) -> U {
        self.message.clone()
    }

    pub fn answer<T>(self, answer: T) -> ServerMessage<T> {
        ServerMessage {
            thread_index: self.thread_index,
            answer,
        }
    }
}

pub struct ServerMessage<T> {
    pub(crate) thread_index: u32,
    pub(crate) answer: T,
}
