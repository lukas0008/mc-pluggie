use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use abi_stable::external_types::crossbeam_channel::RSender;
use dashmap::DashMap;
use mclib_network::{ClientId, ClientMode};
use mclib_protocol::{packet::PacketSerialize, varint::Varint};
use mio::Waker;

use crate::client::Client;

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
pub struct NetworkContextImplementation(pub(crate) Arc<Mutex<NetworkContextInternal>>);

impl NetworkContextImplementation {
    pub fn send_packet(&self, client_id: ClientId, packet: &dyn PacketSerialize) {
        let packet_payload = packet.serialize_packet();
        let mut data = Vec::with_capacity(packet_payload.len() + 4);
        let packet_id_bytes = Varint::new(packet.packet_id()).to_bytes();
        // println!(
        //     "{}",
        //     &packet_payload
        //         .iter()
        //         .map(|v| format!("{v:x}"))
        //         .collect::<Vec<String>>()
        //         .join("")
        // );
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
        if let Some(mut client) = lock.connections.get_mut(&mio::Token(client_id.0)) {
            client.mode = mode;
        }
    }
}
