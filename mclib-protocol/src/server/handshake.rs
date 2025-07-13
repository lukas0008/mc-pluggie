use mc_proc::packet;

use crate::varint::Varint;

#[packet(id = 0x00)]
pub struct SHandshakePacket {
    pub protocol_version: Varint,
    pub server_address: String,
    pub server_port: u16,
    pub intent: Varint,
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Debug)]
#[repr(u8)]
pub enum HandshakeIntent {
    Status = 1,
    Login = 2,
    Transfer = 3,
}
