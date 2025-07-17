mod chunk_data_and_light;
mod game_event;
mod keep_alive;
mod login_play;
mod synchronize_player_position;

pub use game_event::CGameEvent;
pub use keep_alive::CKeepAlive;
pub use login_play::CLoginPlay;
pub use synchronize_player_position::CSynchronizePlayerPosition;

use crate::packet::PacketSerialize;

pub enum CPlayPacket {
    CSynchronizePlayerPosition(CSynchronizePlayerPosition),
    CGameEvent(CGameEvent),
    CLoginPlay(CLoginPlay),
    CKeepAlive(CKeepAlive),
}

#[cfg(feature = "serde")]
impl PacketSerialize for CPlayPacket {
    fn packet_id(&self) -> i32 {
        match self {
            CPlayPacket::CSynchronizePlayerPosition(packet) => packet.packet_id(),
            CPlayPacket::CGameEvent(packet) => packet.packet_id(),
            CPlayPacket::CLoginPlay(packet) => packet.packet_id(),
            CPlayPacket::CKeepAlive(packet) => packet.packet_id(),
        }
    }
    fn serialize_packet(&self) -> Vec<u8> {
        match self {
            CPlayPacket::CSynchronizePlayerPosition(packet) => packet.serialize_packet(),
            CPlayPacket::CGameEvent(packet) => packet.serialize_packet(),
            CPlayPacket::CLoginPlay(packet) => packet.serialize_packet(),
            CPlayPacket::CKeepAlive(packet) => packet.serialize_packet(),
        }
    }
}
