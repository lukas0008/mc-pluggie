use crate::parse_varint;

pub fn get_unparsed_packet_uncompressed(packet: &[u8]) -> Option<(i32, Vec<u8>)> {
    let (packet_id, bytes_taken) = parse_varint(&packet)?;

    Some((packet_id, packet[bytes_taken as usize..].to_vec()))
}
