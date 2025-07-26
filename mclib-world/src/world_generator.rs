use pluggie::reexports::sha2_const;

pub trait WorldGenerator {
    fn name(&self) -> &'static str;
    fn generate_chunk(&self);
}
