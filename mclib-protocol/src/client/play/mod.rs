mod game_event;
mod login_play;
mod synchronize_player_position;
mod chunk_data_and_light;

pub use game_event::CGameEvent;
pub use login_play::CLoginPlay;
pub use synchronize_player_position::CSynchronizePlayerPosition;

use crate::packet::PacketSerialize;

pub enum CPlayPacket {
    CSynchronizePlayerPosition(CSynchronizePlayerPosition),
    CGameEvent(CGameEvent),
    CLoginPlay(CLoginPlay),
}

impl PacketSerialize for CPlayPacket {
    fn packet_id(&self) -> i32 {
        match self {
            CPlayPacket::CSynchronizePlayerPosition(packet) => packet.packet_id(),
            CPlayPacket::CGameEvent(packet) => packet.packet_id(),
            CPlayPacket::CLoginPlay(packet) => packet.packet_id(),
        }
    }
    fn serialize_packet(&self) -> Vec<u8> {
        match self {
            CPlayPacket::CSynchronizePlayerPosition(packet) => packet.serialize_packet(),
            CPlayPacket::CGameEvent(packet) => packet.serialize_packet(),
            CPlayPacket::CLoginPlay(packet) => packet.serialize_packet(),
        }
    }
}
