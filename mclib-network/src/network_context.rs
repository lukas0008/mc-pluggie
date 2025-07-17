use std::{fmt::Debug, sync::Arc};

use mclib_protocol::packet::PacketSerialize;
use pluggie::exposable::Exposable;

use crate::{ClientId, ClientMode};

#[derive(Clone)]
#[repr(C)]
pub struct NetworkContext(pub Arc<NetworkContextFuncs>);
impl Exposable for NetworkContext {
    const NAME: &'static str = "core:mc-network:NetworkContext";
}
impl Debug for NetworkContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("NetworkContext")
    }
}

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
