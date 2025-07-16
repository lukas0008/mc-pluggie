use crate::{heightmap::Heightmap, prefixed_array::PrefixedArray};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ChunkData {
    pub heightmaps: PrefixedArray<Heightmap>,
    pub data: PrefixedArray<u8>,
    // TODO: make something else, for now this array will be empty
    pub block_entities: PrefixedArray<()>,
}
