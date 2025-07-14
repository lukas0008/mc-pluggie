use crate::server::{config::SConfigPacket, handshake::SHandshakePacket, login::SLoginPacket, status::SStatusPacket};

pub mod handshake;
pub mod login;
pub mod status;
pub mod config;

#[derive(Debug)]
pub enum SPacket {
    Status(SStatusPacket),
    Handshake(SHandshakePacket),
    Login(SLoginPacket),
    Config(SConfigPacket),
}
