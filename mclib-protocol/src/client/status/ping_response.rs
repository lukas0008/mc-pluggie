use mc_proc::packet;

#[packet(id = 0x01)]
pub struct CPingResponse {
    pub payload: i64,
}
