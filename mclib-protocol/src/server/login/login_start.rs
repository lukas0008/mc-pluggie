use mc_proc::packet;
use uuid::Uuid;
#[packet(id = 0x00)]
pub struct SLoginStart {
    pub username: String,
    #[serde(with = "uuid::serde::compact")]
    pub uuid: Uuid,
}
