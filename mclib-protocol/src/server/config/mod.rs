mod config_finish_acknowledged;
mod known_packs;

pub use config_finish_acknowledged::SConfigFinishAcknowledged;
pub use known_packs::SKnownPacks;

#[derive(Debug)]
pub enum SConfigPacket {
    SConfigFinishAcknowledged(SConfigFinishAcknowledged),
    SKnownPacks(SKnownPacks),
}
