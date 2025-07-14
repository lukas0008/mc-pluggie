mod login_acknowledged;
mod login_start;

pub use login_acknowledged::SLoginAcknowledged;
pub use login_start::SLoginStart;

#[derive(Debug)]
pub enum SLoginPacket {
    SLoginStart(SLoginStart),
    SLoginAcknowledged(SLoginAcknowledged),
}
