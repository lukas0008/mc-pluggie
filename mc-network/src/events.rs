use pluggie::event::Event;

use crate::client_id::ClientId;

#[repr(C)]
pub struct NewConnectionEvent(pub ClientId);
impl Event for NewConnectionEvent {
    const NAME: &'static str = "core:mc-network:new-connection";
}

#[repr(C)]
pub struct RawPacketEvent {
    pub client_id: ClientId,
    pub data: Vec<u8>,
}
impl Event for RawPacketEvent {
    const NAME: &'static str = "core:mc-network:raw-packet";
}
