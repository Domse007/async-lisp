use bidirchannel::ComClient;

use crate::threadmessage::{EnvToThreadMessage, ThreadToEnvMessage};

pub fn run(endp: ComClient<EnvToThreadMessage, ThreadToEnvMessage>) {
    loop {
        match endp.recv() {
            EnvToThreadMessage::Shutdown => break,
            _ => unreachable!(),
        }
    }
}
