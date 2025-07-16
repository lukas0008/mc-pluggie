use std::sync::{LazyLock, OnceLock};

use mc_registry::registry::Registry;
use mclib_protocol::{client::config::CRegistryData, unprefixed_array::UnprefixedArray};
use simdnbt::Serialize;

pub static REGISTRY_PACKETS: OnceLock<Vec<CRegistryData>> = OnceLock::new();

pub fn populate_registry(registry: &mut Registry) {
    let registry_json = include_str!("../../assets/registry.json");

    let vanilla_registry: Registry = serde_json::from_str(registry_json).unwrap();
    registry.extend(vanilla_registry);
}

pub fn populate_registry_packets(registry: &Registry) {
    let mut packets = Vec::new();

    let mut dims = Vec::new();
    for (name, data) in &registry.dimension_types {
        let data = data.clone();
        let nbt = data.to_nbt();
        let mut buf = Vec::new();
        nbt.write_unnamed(&mut buf);
        dims.push((name.to_string(), Some(UnprefixedArray(buf))));
    }

    packets.push(CRegistryData {
        registry_id: "minecraft:dimension_type".into(),
        entries: dims.into(),
    });

    let _ = REGISTRY_PACKETS.set(packets);
}
