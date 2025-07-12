use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use abi_stable::external_types::crossbeam_channel::RSender;
use mio::Waker;
use pluggie::exposable::Exposable;

use crate::ClientId;

#[derive(Debug)]
pub(crate) enum NetworkTask {
    SendPacket(ClientId, Vec<u8>),
}

#[repr(C)]
pub(crate) struct NetworkContextInternal {
    pub task_sender: RSender<NetworkTask>,
    pub waker: Arc<Waker>,
    pub yo: u32,
}

#[derive(Clone)]
#[repr(C)]
pub struct NetworkContext(pub(crate) Arc<Mutex<NetworkContextInternal>>);

impl Debug for NetworkContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("NetworkContext")
    }
}

impl NetworkContext {
    pub fn send_raw_packet(&self, client_id: ClientId, packet: Vec<u8>) {
        let lock = self.0.lock().unwrap();
        lock.task_sender
            .send(NetworkTask::SendPacket(client_id, packet))
            .unwrap();
        lock.waker.wake().unwrap();
    }
}
impl Exposable for NetworkContext {
    const NAME: &'static str = "core:mc-network:NetworkContext";
}
