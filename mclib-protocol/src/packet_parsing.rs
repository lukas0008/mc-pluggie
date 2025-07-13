use crate::varint::Varint;

pub fn get_unparsed_packet_uncompressed(packet: &[u8]) -> Option<(i32, Vec<u8>)> {
    let (packet_id, bytes_taken) = Varint::parse(&packet)?;
    Some((packet_id.0, packet[bytes_taken as usize..].to_vec()))
}
