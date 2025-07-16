use mc_proc::packet;

use crate::prefixed_array::PrefixedArray;

#[packet(id = 0x07)]
pub struct SKnownPacks {
    pub known_packs: PrefixedArray<(String, String, String)>,
}
