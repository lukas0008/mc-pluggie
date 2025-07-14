use crate::{
    client::{
        config::CConfigPacket, login::CLoginPacket, play::CPlayPacket, status::CStatusPacket,
    },
    packet::PacketSerialize,
};
pub mod config;
pub mod login;
pub mod play;
pub mod status;

pub enum CPacket {
    Status(CStatusPacket),
    Login(CLoginPacket),
    Config(CConfigPacket),
    Play(CPlayPacket),
}

impl PacketSerialize for CPacket {
    fn serialize_packet(&self) -> Vec<u8> {
        match self {
            CPacket::Status(packet) => packet.serialize_packet(),
            CPacket::Login(packet) => packet.serialize_packet(),
            CPacket::Config(packet) => packet.serialize_packet(),
            CPacket::Play(packet) => packet.serialize_packet(),
        }
    }
    fn packet_id(&self) -> i32 {
        match self {
            CPacket::Status(packet) => packet.packet_id(),
            CPacket::Login(packet) => packet.packet_id(),
            CPacket::Config(packet) => packet.packet_id(),
            CPacket::Play(packet) => packet.packet_id(),
        }
    }
}
