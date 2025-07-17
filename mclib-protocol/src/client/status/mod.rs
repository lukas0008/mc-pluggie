mod ping_response;
pub mod status_response;

pub use ping_response::CPingResponse;
pub use status_response::CStatusResponse;

use crate::{Packet, packet::PacketSerialize};

pub enum CStatusPacket {
    CPingResponse(CPingResponse),
    CStatusResponse(CStatusResponse),
}

#[cfg(feature = "serde")]
impl PacketSerialize for CStatusPacket {
    fn serialize_packet(&self) -> Vec<u8> {
        match self {
            CStatusPacket::CPingResponse(packet) => packet.serialize_packet(),
            CStatusPacket::CStatusResponse(packet) => packet.serialize_packet(),
        }
    }
    fn packet_id(&self) -> i32 {
        match self {
            CStatusPacket::CPingResponse(_) => CPingResponse::PACKET_ID,
            CStatusPacket::CStatusResponse(_) => CStatusResponse::PACKET_ID,
        }
    }
}
