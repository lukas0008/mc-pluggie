pub trait PacketSerialize {
    fn packet_id(&self) -> i32;
    fn serialize_packet(&self) -> Vec<u8>;
}

pub trait Packet {
    const PACKET_ID: i32;
}
