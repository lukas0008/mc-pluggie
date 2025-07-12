mod ping_response;
pub use ping_response::CPingResponse;

pub enum CStatusPacket {
    PingResponse(CPingResponse),
}
