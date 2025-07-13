use crate::server::{handshake::SHandshakePacket, status::SStatusPacket};

pub mod handshake;
pub mod status;

#[derive(Debug)]
pub enum SPacket {
    Status(SStatusPacket),
    Handshake(SHandshakePacket),
}
