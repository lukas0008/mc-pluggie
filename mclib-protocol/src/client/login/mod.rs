mod login_success;

pub use login_success::CLoginSuccess;

use crate::packet::PacketSerialize;

pub enum CLoginPacket {
    CLoginSuccess(CLoginSuccess),
}

#[cfg(feature = "serde")]
impl PacketSerialize for CLoginPacket {
    fn packet_id(&self) -> i32 {
        match self {
            CLoginPacket::CLoginSuccess(packet) => packet.packet_id(),
        }
    }
    fn serialize_packet(&self) -> Vec<u8> {
        match self {
            CLoginPacket::CLoginSuccess(packet) => packet.serialize_packet(),
        }
    }
}
