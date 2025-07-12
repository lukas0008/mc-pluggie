pub mod client;
pub mod packet;
pub use packet::Packet;
pub mod packet_parsing;
pub mod server;

/// Parse a variable-length integer from a byte slice.
/// Returns the parsed integer and the number of bytes read.
pub fn parse_varint(bytes: &[u8]) -> Option<(i32, u8)> {
    const CONTINUE: u8 = 0b10000000;
    const MASK: u8 = 0b01111111;

    let mut value = 0i32;

    for i in 0..5 {
        let byte = bytes.get(i)?;
        value |= ((byte & MASK) as i32) << (i * 7);
        if byte & CONTINUE == 0 {
            return Some((value, i as u8 + 1));
        }
    }

    None
}

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
