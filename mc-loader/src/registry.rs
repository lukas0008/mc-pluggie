use std::sync::OnceLock;

use mc_registry::{dimension_type::MonsterSpawnLightLevel, registry::Registry};
use mclib_protocol::{
    client::config::CRegistryData, prefixed_array::PrefixedArray, unprefixed_array::UnprefixedArray,
};

pub static REGISTRY_PACKETS: OnceLock<Vec<CRegistryData>> = OnceLock::new();

pub fn populate_registry(registry: &mut Registry) {
    let registry_json = include_str!("../../assets/registry.json");

    let vanilla_registry: Registry = serde_json::from_str(registry_json).unwrap();
    registry.extend(vanilla_registry);
}

pub fn populate_registry_packets(registry: &Registry) {
    fn do_stuff<T: simdnbt::Serialize + Clone>(
        stuff: &Vec<(String, T)>,
    ) -> PrefixedArray<(String, Option<UnprefixedArray<u8>>)> {
        let mut entries = Vec::new();
        for (name, data) in stuff {
            let data = data.clone();
            let nbt = data.to_nbt();
            let mut buf = Vec::new();
            nbt.write_unnamed(&mut buf);
            entries.push((name.to_string(), Some(UnprefixedArray(buf))));
        }
        entries.into()
    }
    let mut packets = Vec::new();

    packets.push(CRegistryData {
        registry_id: "minecraft:dimension_type".into(),
        entries: do_stuff(
            &registry
                .dimension_types
                .iter()
                .map(|(a, b)| {
                    let mut b = b.clone();
                    b.monster_spawn_light_level = MonsterSpawnLightLevel::Fixed(7);
                    (a.clone(), b)
                })
                .collect(),
        ),
    });

    packets.push(CRegistryData {
        registry_id: "minecraft:chicken_variant".into(),
        entries: do_stuff(&registry.chicken_variants),
    });

    packets.push(CRegistryData {
        registry_id: "minecraft:cow_variant".into(),
        entries: do_stuff(&registry.cow_variants),
    });

    packets.push(CRegistryData {
        registry_id: "minecraft:frog_variant".into(),
        entries: do_stuff(&registry.frog_variants),
    });

    packets.push(CRegistryData {
        registry_id: "minecraft:painting_variant".into(),
        entries: do_stuff(&registry.painting_variants),
    });

    packets.push(CRegistryData {
        registry_id: "minecraft:pig_variant".into(),
        entries: do_stuff(&registry.pig_variants),
    });

    packets.push(CRegistryData {
        registry_id: "minecraft:wolf_sound_variant".into(),
        entries: do_stuff(&registry.wolf_sound_variants),
    });

    packets.push(CRegistryData {
        registry_id: "minecraft:wolf_variant".into(),
        entries: do_stuff(&registry.wolf_variants),
    });

    packets.push(CRegistryData {
        registry_id: "minecraft:cat_variant".into(),
        entries: do_stuff(&registry.cat_variants),
    });

    let _ = REGISTRY_PACKETS.set(packets);
}
