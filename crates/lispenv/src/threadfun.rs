use bidirchannel::BiDirChannel;

use crate::threadmessage::ThreadMessage;

pub fn run(endp: BiDirChannel<ThreadMessage>) {
    loop {
        match endp.recv() {
            ThreadMessage::Shutdown => return,
            _ => unreachable!(),
        }
    }
}
