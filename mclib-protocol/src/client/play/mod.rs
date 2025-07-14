mod game_event;
mod synchronize_player_position;

pub use game_event::CGameEvent;
pub use synchronize_player_position::CSynchronizePlayerPosition;

use crate::packet::PacketSerialize;

pub enum CPlayPacket {
    CSynchronizePlayerPosition(CSynchronizePlayerPosition),
    CGameEvent(CGameEvent),
}

impl PacketSerialize for CPlayPacket {
    fn packet_id(&self) -> i32 {
        match self {
            CPlayPacket::CSynchronizePlayerPosition(packet) => packet.packet_id(),
            CPlayPacket::CGameEvent(packet) => packet.packet_id(),
        }
    }
    fn serialize_packet(&self) -> Vec<u8> {
        match self {
            CPlayPacket::CSynchronizePlayerPosition(packet) => packet.serialize_packet(),
            CPlayPacket::CGameEvent(packet) => packet.serialize_packet(),
        }
    }
}
