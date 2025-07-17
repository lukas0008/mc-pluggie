use crate::prefixed_array::PrefixedArray;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub struct LightData {
    pub sky_light_mask: PrefixedArray<i64>,
    pub block_light_mask: PrefixedArray<i64>,
    pub empty_sky_mask: PrefixedArray<i64>,
    pub empty_block_light_mask: PrefixedArray<i64>,
    pub sky_light: PrefixedArray<u8>,
    pub block_light: PrefixedArray<u8>,
}
