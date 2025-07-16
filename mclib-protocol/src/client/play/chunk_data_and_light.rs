use mc_proc::packet;

use crate::{chunk_data::ChunkData, light_data::LightData};

#[packet(id = 0x27, serialize_only = true)]
pub struct CChunkDataAndLight {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub chunk_data: ChunkData,
    pub light_data: LightData,
}
