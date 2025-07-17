use crate::{prefixed_array::PrefixedArray, varint::Varint};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub struct Heightmap {
    pub kind: Varint,
    pub heightmap: PrefixedArray<i64>,
}
