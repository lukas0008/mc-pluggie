use mc_proc::packet;

use crate::{prefixed_array::PrefixedArray, varint::Varint};

#[packet(id = 0x2b)]
pub struct CLoginPlay {
    pub entity_id: i32,
    pub hardcore: bool,
    pub dimensions: PrefixedArray<String>,
    pub max_players: Varint,
    pub view_distance: Varint,
    pub simulation_distance: Varint,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub do_limited_crafting: bool,
    pub dimension_type: Varint,
    pub dimension_name: String,
    pub hashed_seed: i64,
    pub game_mode: u8,
    pub previous_game_mode: i8,
    pub is_debug: bool,
    pub is_flat: bool,
    pub death_location: Option<(String, u64)>,
    pub portal_cooldown: Varint,
    pub sea_level: Varint,
    pub enforces_secure_chat: bool,
}
