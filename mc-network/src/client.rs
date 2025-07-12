use std::net::SocketAddr;

use mclib_protocol::parse_varint;
use pluggie::pluggie_context::EventSender;

use crate::{client_id::ClientId, events::RawPacketEvent};

#[derive(Debug)]
pub struct Client {
    pub id: ClientId,
    pub conn: mio::net::TcpStream,
    #[expect(unused)]
    pub addr: SocketAddr,
    pub currently_writable: bool,
    pub to_write: Vec<u8>,
    pub read_buffer: Vec<u8>,
    pub read_buffer_bytes: usize,
}

impl Client {
    /// This function updates the internal read buffer with the given bytes, additionally it also parses the bytes to get RawPacket's, calling this function with an empty buffer will parse the bytes remaining in the buffer
    pub fn update_received_bytes(
        &mut self,
        bytes: &[u8],
        raw_packet_event_sender: &EventSender<RawPacketEvent>,
    ) {
        self.read_buffer.extend_from_slice(bytes);

        loop {
            let (len, bytes_used_on_varint) = if let Some(v) = parse_varint(&self.read_buffer) {
                v
            } else {
                return;
            };

            let total_bytes_used = bytes_used_on_varint as usize + len as usize;

            if total_bytes_used > self.read_buffer.len() {
                return;
            }

            let _ = self.read_buffer.drain(..bytes_used_on_varint as usize);
            let data = self.read_buffer.drain(..len as usize).collect::<Vec<u8>>();
            raw_packet_event_sender.call(&RawPacketEvent {
                client_id: self.id,
                data,
            });
        }
    }
}
