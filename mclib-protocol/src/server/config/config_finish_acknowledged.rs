use mc_proc::packet;

#[packet(id = 0x03)]
pub struct SConfigFinishAcknowledged {}

impl SConfigFinishAcknowledged {
    pub fn new() -> Self {
        SConfigFinishAcknowledged {}
    }
}
