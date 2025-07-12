use crate::server::status::SStatusPacket;

pub mod status;

pub enum SPacket {
    Status(SStatusPacket),
}
