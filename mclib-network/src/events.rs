use mclib_protocol::SPacket;
use pluggie::event::Event;

use crate::{client_id::ClientId, client_mode::ClientMode};

#[repr(C)]
pub struct NewConnectionEvent(pub ClientId);
impl Event for NewConnectionEvent {
    const NAME: &'static str = "core:mc-network:new-connection";
}

#[repr(C)]
pub struct RawPacketEvent {
    pub client_id: ClientId,
    pub client_mode: ClientMode,
    pub data: Vec<u8>,
}
impl Event for RawPacketEvent {
    const NAME: &'static str = "core:mc-network:raw-packet";
}

#[derive(Debug)]
#[repr(C)]
pub struct ServerPacketEvent {
    pub client_id: ClientId,
    pub packet: SPacket,
}

impl Event for ServerPacketEvent {
    const NAME: &'static str = "core:mc-network:server-packet";
}
