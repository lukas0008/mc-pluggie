use mc_proc::packet;

#[packet(id = 0x1e)]
pub struct SSetPlayerPosRot {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: i8,
}
