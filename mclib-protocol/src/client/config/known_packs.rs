use mc_proc::packet;

use crate::prefixed_array::PrefixedArray;

#[packet(id = 0x0e)]
pub struct CKnownPacks {
    pub known_packs: PrefixedArray<(String, String, String)>,
}
