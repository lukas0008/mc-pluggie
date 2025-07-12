mod ping_request;

pub use ping_request::SPingRequest;

pub enum SStatusPacket {
    PingRequest(SPingRequest),
}
