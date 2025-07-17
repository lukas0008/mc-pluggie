mod set_player_pos;
mod set_player_pos_rot;
mod tick_end;

pub use set_player_pos::SSetPlayerPos;
pub use set_player_pos_rot::SSetPlayerPosRot;
pub use tick_end::STickEnd;

#[derive(Debug)]
pub enum SPlayPacket {
    SSetPlayerPos(SSetPlayerPos),
    STickEnd(STickEnd),
    SSetPlayerPosRot(SSetPlayerPosRot),
}
