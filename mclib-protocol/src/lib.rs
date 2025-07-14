pub mod client;
pub mod packet;
pub use client::CPacket;
pub use packet::Packet;
pub use server::SPacket;
pub mod packet_parsing;
pub mod prefixed_array;
#[cfg(feature = "serde")]
pub mod serde;
pub mod server;
pub mod varint;
pub mod property;

// TODO: replace with varlong struct
pub fn parse_varlong(bytes: &[u8]) -> Option<i64> {
    const CONTINUE: u8 = 0b10000000;
    const MASK: u8 = 0b01111111;

    let mut value = 0i64;

    for i in 0..10 {
        let byte = bytes.get(i)?;
        value |= ((byte & MASK) as i64) << (i * 7);
        if byte & CONTINUE == 0 {
            return Some(value);
        }
    }

    None
}
