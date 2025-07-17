use crate::server::{
    config::SConfigPacket, handshake::SHandshakePacket, login::SLoginPacket, play::SPlayPacket,
    status::SStatusPacket,
};

pub mod config;
pub mod handshake;
pub mod login;
pub mod play;
pub mod status;

#[derive(Debug)]
pub enum SPacket {
    Status(SStatusPacket),
    Handshake(SHandshakePacket),
    Login(SLoginPacket),
    Config(SConfigPacket),
    Play(SPlayPacket),
}
