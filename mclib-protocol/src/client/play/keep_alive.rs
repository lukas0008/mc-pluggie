use mc_proc::packet;

#[packet(id = 0x26)]
pub struct CKeepAlive {
    pub id: i64,
}
