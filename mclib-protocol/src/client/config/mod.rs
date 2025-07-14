mod finish_config;
mod known_packs;

pub use finish_config::CFinishConfig;
pub use known_packs::CKnownPacks;

use crate::packet::PacketSerialize;

pub enum CConfigPacket {
    CFinishConfig(CFinishConfig),
    CKnownPacks(CKnownPacks),
}

impl PacketSerialize for CConfigPacket {
    fn packet_id(&self) -> i32 {
        match self {
            CConfigPacket::CFinishConfig(packet) => packet.packet_id(),
            CConfigPacket::CKnownPacks(packet) => packet.packet_id(),
        }
    }
    fn serialize_packet(&self) -> Vec<u8> {
        match self {
            CConfigPacket::CFinishConfig(packet) => packet.serialize_packet(),
            CConfigPacket::CKnownPacks(packet) => packet.serialize_packet(),
        }
    }
}
