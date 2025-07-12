use crate::client::status::CStatusPacket;
pub mod status;

pub enum CPacket {
    Status(CStatusPacket),
}
