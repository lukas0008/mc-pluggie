mod finish_config;
mod known_packs;
mod registry_data;

pub use finish_config::CFinishConfig;
pub use known_packs::CKnownPacks;
pub use registry_data::CRegistryData;

use crate::packet::PacketSerialize;

pub enum CConfigPacket {
    CFinishConfig(CFinishConfig),
    CKnownPacks(CKnownPacks),
    CRegistryData(CRegistryData),
}

impl PacketSerialize for CConfigPacket {
    fn packet_id(&self) -> i32 {
        match self {
            CConfigPacket::CFinishConfig(packet) => packet.packet_id(),
            CConfigPacket::CKnownPacks(packet) => packet.packet_id(),
            CConfigPacket::CRegistryData(packet) => packet.packet_id(),
        }
    }
    fn serialize_packet(&self) -> Vec<u8> {
        match self {
            CConfigPacket::CFinishConfig(packet) => packet.serialize_packet(),
            CConfigPacket::CKnownPacks(packet) => packet.serialize_packet(),
            CConfigPacket::CRegistryData(packet) => packet.serialize_packet(),
        }
    }
}
