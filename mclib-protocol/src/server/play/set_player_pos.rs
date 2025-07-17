use mc_proc::packet;

#[packet(id = 0x1d)]
pub struct SSetPlayerPos {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub flags: i8,
}
