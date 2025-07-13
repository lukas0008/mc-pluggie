pub mod client;
pub mod packet;
pub use client::CPacket;
pub use packet::Packet;
pub use server::SPacket;
pub mod packet_parsing;
pub mod serde;
pub mod server;
pub mod varint;

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

// TODO: replace with varint method
fn varint_to_bytes(mut value: i32) -> Vec<u8> {
    let mut bytes = Vec::new();

    for _ in 0..5 {
        const CONTINUE: u8 = 0b10000000;
        const MASK: u8 = 0b01111111;

        let mut byte = (value & MASK as i32) as u8;

        value >>= 7;
        if value > 0 {
            byte |= CONTINUE;
        }
        bytes.push(byte);

        if value == 0 {
            break;
        }
    }

    bytes
}
