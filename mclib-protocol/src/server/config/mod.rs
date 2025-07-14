mod config_finish_acknowledged;

pub use config_finish_acknowledged::SConfigFinishAcknowledged;

#[derive(Debug)]
pub enum SConfigPacket {
    SConfigFinishAcknowledged(SConfigFinishAcknowledged),
}
