#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct DimensionType {
    pub fixed_time: Option<i64>,
    pub has_skylight: bool,
    pub has_ceiling: bool,
    pub ultrawarm: bool,
    pub natural: bool,
    pub coordinate_scale: f64,
    pub bed_works: bool,
    pub respawn_anchor_works: bool,
    pub min_y: i32,
    pub height: i32,
    pub logical_height: i32,
    pub infiniburn: String,
    pub effects: String,
    pub ambient_light: f32,
    pub piglin_safe: bool,
    pub has_raids: bool,
    pub monster_spawn_light_level: MonsterSpawnLightLevel,
    pub monster_spawn_block_light_limit: i32,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum MonsterSpawnLightLevel {
    Fixed(i32),
    Distribution {
        max_inclusive: i32,
        min_inclusive: i32,
        #[serde(rename = "type")]
        kind: String,
    },
}

impl simdnbt::Serialize for DimensionType {
    fn to_compound(self) -> simdnbt::owned::NbtCompound {
        let mut comp = simdnbt::owned::NbtCompound::new();

        if let Some(fixed_time) = self.fixed_time {
            comp.insert("fixed_time", fixed_time);
        }
        comp.insert("has_skylight", self.has_skylight);
        comp.insert("has_ceiling", self.has_ceiling);
        comp.insert("ultrawarm", self.ultrawarm);
        comp.insert("natural", self.natural);
        comp.insert("coordinate_scale", self.coordinate_scale);
        comp.insert("bed_works", self.bed_works);
        comp.insert("respawn_anchor_works", self.respawn_anchor_works);
        comp.insert("min_y", self.min_y);
        comp.insert("height", self.height);
        comp.insert("logical_height", self.logical_height);
        comp.insert("infiniburn", self.infiniburn);
        comp.insert("effects", self.effects);
        comp.insert("ambient_light", self.ambient_light);
        comp.insert("piglin_safe", self.piglin_safe);
        comp.insert("has_raids", self.has_raids);
        match self.monster_spawn_light_level {
            MonsterSpawnLightLevel::Fixed(v) => {
                comp.insert("monster_spawn_light_level", v);
            }
            MonsterSpawnLightLevel::Distribution {
                min_inclusive,
                max_inclusive,
                kind,
            } => {
                let mut comp2 = simdnbt::owned::NbtCompound::new();
                comp2.insert("min", min_inclusive);
                comp2.insert("max", max_inclusive);
                comp2.insert("type", kind);
                comp.insert("monster_spawn_light_level", comp2);
            }
        }
        comp.insert(
            "monster_spawn_block_light_limit",
            self.monster_spawn_block_light_limit,
        );

        comp
    }
}
