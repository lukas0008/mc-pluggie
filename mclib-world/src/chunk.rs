use mclib_core::vector3::BlockPos;

use crate::BlockId;

pub struct Chunk {
    pub pos: (i32, i32),
    pub min_height: i32,
    pub chunk_sections: Vec<ChunkSection>,
}

pub enum ChunkSection {
    Uniform(BlockId),
    Paletted {
        palette: Vec<BlockId>,
        blocks: Vec<u64>,
    },
}

impl Chunk {
    // TODO: should prob replace with a result
    pub fn get_block_rel(&self, pos: BlockPos) -> Option<BlockId> {
        if pos.x >= 16
            || pos.z >= 16
            || pos.x < 0
            || pos.z < 0
            || pos.y < self.min_height
            || pos.y > self.chunk_sections.len() as i32 * 16 + self.min_height
        {
            return None;
        }
        let y = pos.y - self.min_height;
        let section = y / 16;
        let section = &self.chunk_sections[section as usize];
        match section {
            ChunkSection::Uniform(id) => Some(*id),
            ChunkSection::Paletted { palette, blocks } => {
                let 
            }
        }
    }
}
