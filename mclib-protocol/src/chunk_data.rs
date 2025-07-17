use crate::{heightmap::Heightmap, prefixed_array::PrefixedArray};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub struct ChunkData {
    pub heightmaps: PrefixedArray<Heightmap>,
    pub data: PrefixedArray<u8>,
    // TODO: make something else, for now this array will be empty
    pub block_entities: PrefixedArray<()>,
}
