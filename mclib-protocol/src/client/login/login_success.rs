use mc_proc::packet;
use uuid::Uuid;

use crate::{prefixed_array::PrefixedArray, property::Property};

#[packet(id = 0x02)]
pub struct CLoginSuccess {
    #[serde(with = "uuid::serde::compact")]
    pub uuid: Uuid,
    pub username: String,
    pub properties: PrefixedArray<Property>,
}
