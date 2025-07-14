use mc_proc::packet;

#[packet(id = 0x22)]
pub struct CGameEvent {
    pub event: u8,
    pub value: f32,
}
