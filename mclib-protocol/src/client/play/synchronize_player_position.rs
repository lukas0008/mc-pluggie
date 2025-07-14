use mc_proc::packet;

use crate::varint::Varint;

#[packet(id = 0x41)]
pub struct CSynchronizePlayerPosition {
    pub tp_id: Varint,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vel_x: f64,
    pub vel_y: f64,
    pub vel_z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: u32,
}
