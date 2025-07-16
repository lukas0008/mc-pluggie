use mc_proc::packet;

use crate::{prefixed_array::PrefixedArray, unprefixed_array::UnprefixedArray};

#[packet(id = 0x07, serialize_only = true)]
pub struct CRegistryData {
    pub registry_id: String,
    pub entries: PrefixedArray<(String, Option<UnprefixedArray<u8>>)>,
}
