mod ping_request;
mod status_request;

pub use ping_request::SPingRequest;
pub use status_request::SStatusRequest;

#[derive(Debug)]
pub enum SStatusPacket {
    SPingRequest(SPingRequest),
    SStatusRequest(SStatusRequest),
}
