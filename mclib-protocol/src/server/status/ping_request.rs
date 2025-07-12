use mc_proc::packet;

#[packet(id = 0x01)]
pub struct SPingRequest {
    pub payload: i64,
}
