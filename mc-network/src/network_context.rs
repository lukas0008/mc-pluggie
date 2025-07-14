use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use abi_stable::external_types::crossbeam_channel::RSender;
use dashmap::DashMap;
use mclib_protocol::{CPacket, packet::PacketSerialize, varint::Varint};
use mio::Waker;
use pluggie::exposable::Exposable;

use crate::{ClientId, client::Client, client_mode::ClientMode};

#[derive(Debug)]
pub(crate) enum NetworkTask {
    SendPacket(ClientId, Vec<u8>),
    CloseClient(ClientId),
}

#[repr(C)]
pub(crate) struct NetworkContextInternal {
    pub task_sender: RSender<NetworkTask>,
    pub waker: Arc<Waker>,
    pub connections: Arc<DashMap<mio::Token, Client>>,
}

#[derive(Clone)]
#[repr(C)]
pub struct NetworkContext(pub Arc<NetworkContextFuncs>);

impl NetworkContext {
    pub fn send_packet(&self, client_id: ClientId, packet: &dyn PacketSerialize) {
        (self.0.send_packet)(client_id, packet);
    }

    pub fn close_client(&self, client_id: ClientId) {
        (self.0.close_client)(client_id);
    }

    pub fn send_raw_packet(&self, client_id: ClientId, packet: Vec<u8>) {
        (self.0.send_raw_packet)(client_id, packet);
    }

    pub fn switch_client_mode(&self, client_id: ClientId, mode: ClientMode) {
        (self.0.switch_client_mode)(client_id, mode);
    }
}

#[repr(C)]
pub struct NetworkContextFuncs {
    pub send_packet: Box<dyn Fn(ClientId, &dyn PacketSerialize) + Send + Sync>,
    pub close_client: Box<dyn Fn(ClientId) + Send + Sync>,
    pub send_raw_packet: Box<dyn Fn(ClientId, Vec<u8>) + Send + Sync>,
    pub switch_client_mode: Box<dyn Fn(ClientId, ClientMode) + Send + Sync>,
}

#[derive(Clone)]
#[repr(C)]
pub struct NetworkContextImplementation(pub(crate) Arc<Mutex<NetworkContextInternal>>);

impl NetworkContextFuncs {
    pub fn send_packet(&self, client_id: ClientId, packet: &dyn PacketSerialize) {
        (self.send_packet)(client_id, packet);
    }
    pub fn close_client(&self, client_id: ClientId) {
        (self.close_client)(client_id);
    }
    pub fn send_raw_packet(&self, client_id: ClientId, packet: Vec<u8>) {
        (self.send_raw_packet)(client_id, packet);
    }
    pub fn switch_client_mode(&self, client_id: ClientId, mode: ClientMode) {
        (self.switch_client_mode)(client_id, mode);
    }
}

impl Debug for NetworkContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("NetworkContext")
    }
}

impl NetworkContextImplementation {
    pub fn send_packet(&self, client_id: ClientId, packet: &dyn PacketSerialize) {
        let packet_payload = packet.serialize_packet();
        let mut data = Vec::with_capacity(packet_payload.len() + 4);
        let packet_id_bytes = Varint::new(packet.packet_id()).to_bytes();
        data.extend(Varint::new((packet_payload.len() + packet_id_bytes.len()) as i32).to_bytes());
        data.extend(packet_id_bytes);
        data.extend(packet_payload);
        self.send_raw_packet(client_id, data);
    }
    pub fn close_client(&self, client_id: ClientId) {
        let lock = self.0.lock().unwrap();
        lock.task_sender
            .send(NetworkTask::CloseClient(client_id))
            .unwrap();
        lock.waker.wake().unwrap();
    }
    pub fn send_raw_packet(&self, client_id: ClientId, packet: Vec<u8>) {
        let lock = self.0.lock().unwrap();
        lock.task_sender
            .send(NetworkTask::SendPacket(client_id, packet))
            .unwrap();
        lock.waker.wake().unwrap();
    }
    pub fn switch_client_mode(&self, client_id: ClientId, mode: ClientMode) {
        let lock = self.0.lock().unwrap();
        if let Some(mut client) = lock.connections.get_mut(&client_id.as_token()) {
            client.mode = mode;
        }
    }
}
impl Exposable for NetworkContext {
    const NAME: &'static str = "core:mc-network:NetworkContext";
}
