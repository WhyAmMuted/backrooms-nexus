use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

pub type NetworkAudioPacket = [f32; 960];
const MAX_PACKETS_IN_BUFFER: usize = 4;

pub struct NetworkAudioSender {
    queue: Arc<Mutex<VecDeque<NetworkAudioPacket>>>,
}

impl NetworkAudioSender {
    pub fn push(&mut self, packet: NetworkAudioPacket) -> bool {
        if let Ok(mut queue) = self.queue.try_lock() {
            queue.push_front(packet);
            if queue.len() > MAX_PACKETS_IN_BUFFER {
                queue.pop_back();
            }
            return true;
        }
        false
    }
}

pub struct NetworkAudioReceiver {
    queue: Arc<Mutex<VecDeque<NetworkAudioPacket>>>,
}

impl NetworkAudioReceiver {
    pub fn get(&mut self) -> Option<NetworkAudioPacket> {
        if let Ok(mut queue) = self.queue.try_lock() {
            if queue.len() == 0 {
                return None;
            }
            return queue.pop_back();
        }
        None
    }
}

pub fn create_network_stream() -> (NetworkAudioSender, NetworkAudioReceiver) {
    let queue = Arc::new(Mutex::new(VecDeque::with_capacity(MAX_PACKETS_IN_BUFFER)));
    (
        NetworkAudioSender {
            queue: queue.clone(),
        },
        NetworkAudioReceiver { queue },
    )
}
