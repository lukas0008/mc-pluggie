mod ping_request;

pub use ping_request::SPingRequest;

#[derive(Debug)]
pub enum SStatusPacket {
    SPingRequest(SPingRequest),
}
