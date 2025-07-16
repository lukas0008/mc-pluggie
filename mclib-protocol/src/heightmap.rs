use crate::{prefixed_array::PrefixedArray, varint::Varint};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Heightmap {
    pub kind: Varint,
    pub heightmap: PrefixedArray<i64>,
}
