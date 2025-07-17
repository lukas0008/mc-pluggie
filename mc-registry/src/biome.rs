#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Biome {
    pub has_precipitation: bool,
    pub temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_modifier: Option<String>,
    pub downfall: f32,
    pub effects: BiomeEffects,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct BiomeEffects {
    pub mood_sound: MoodSound,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music: Option<Vec<Music>>,
    pub music_volume: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additions_sound: Option<AdditionsSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle: Option<Particle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ambient_sound: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grass_color_modifier: Option<String>,
    pub sky_color: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foliage_color: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grass_color: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_foliage_color: Option<u32>,
    pub fog_color: u32,
    pub water_color: u32,
    pub water_fog_color: u32,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct MoodSound {
    pub sound: String,
    pub tick_delay: u32,
    pub block_search_extent: u32,
    pub offset: f32,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Music {
    pub data: MusicData,
    pub weight: u32,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct MusicData {
    pub sound: String,
    pub min_delay: u32,
    pub max_delay: u32,
    pub replace_current_music: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct AdditionsSound {
    pub sound: String,
    pub tick_chance: f32,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Particle {
    pub options: ParticleOptions,
    pub probability: f32,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ParticleOptions {
    #[serde(rename = "type")]
    pub particle_type: String,
}

impl simdnbt::Serialize for Biome {
    fn to_compound(self) -> simdnbt::owned::NbtCompound {
        let mut comp = simdnbt::owned::NbtCompound::new();

        comp.insert("has_precipitation", self.has_precipitation);
        comp.insert("temperature", self.temperature);
        comp.insert("downfall", self.downfall);

        if let Some(temperature_modifier) = self.temperature_modifier {
            comp.insert("temperature_modifier", temperature_modifier);
        }

        comp.insert("effects", self.effects.to_compound());

        comp
    }
}

impl simdnbt::Serialize for BiomeEffects {
    fn to_compound(self) -> simdnbt::owned::NbtCompound {
        let mut comp = simdnbt::owned::NbtCompound::new();

        comp.insert("mood_sound", self.mood_sound.to_compound());
        comp.insert("music_volume", self.music_volume);
        comp.insert("sky_color", self.sky_color as i32);
        comp.insert("fog_color", self.fog_color as i32);
        comp.insert("water_color", self.water_color as i32);
        comp.insert("water_fog_color", self.water_fog_color as i32);

        if let Some(music) = self.music {
            let music_list: Vec<simdnbt::owned::NbtCompound> =
                music.into_iter().map(|m| m.to_compound()).collect();
            comp.insert("music", music_list);
        }

        if let Some(additions_sound) = self.additions_sound {
            comp.insert("additions_sound", additions_sound.to_compound());
        }

        if let Some(particle) = self.particle {
            comp.insert("particle", particle.to_compound());
        }

        if let Some(ambient_sound) = self.ambient_sound {
            comp.insert("ambient_sound", ambient_sound);
        }

        if let Some(grass_color_modifier) = self.grass_color_modifier {
            comp.insert("grass_color_modifier", grass_color_modifier);
        }

        if let Some(foliage_color) = self.foliage_color {
            comp.insert("foliage_color", foliage_color as i32);
        }

        if let Some(grass_color) = self.grass_color {
            comp.insert("grass_color", grass_color as i32);
        }

        if let Some(dry_foliage_color) = self.dry_foliage_color {
            comp.insert("dry_foliage_color", dry_foliage_color as i32);
        }

        comp
    }
}

impl simdnbt::Serialize for MoodSound {
    fn to_compound(self) -> simdnbt::owned::NbtCompound {
        let mut comp = simdnbt::owned::NbtCompound::new();

        comp.insert("sound", self.sound);
        comp.insert("tick_delay", self.tick_delay as i32);
        comp.insert("block_search_extent", self.block_search_extent as i32);
        comp.insert("offset", self.offset);

        comp
    }
}

impl simdnbt::Serialize for Music {
    fn to_compound(self) -> simdnbt::owned::NbtCompound {
        let mut comp = simdnbt::owned::NbtCompound::new();

        comp.insert("data", self.data.to_compound());
        comp.insert("weight", self.weight as i32);

        comp
    }
}

impl simdnbt::Serialize for MusicData {
    fn to_compound(self) -> simdnbt::owned::NbtCompound {
        let mut comp = simdnbt::owned::NbtCompound::new();

        comp.insert("sound", self.sound);
        comp.insert("min_delay", self.min_delay as i32);
        comp.insert("max_delay", self.max_delay as i32);
        comp.insert("replace_current_music", self.replace_current_music);

        comp
    }
}

impl simdnbt::Serialize for AdditionsSound {
    fn to_compound(self) -> simdnbt::owned::NbtCompound {
        let mut comp = simdnbt::owned::NbtCompound::new();

        comp.insert("sound", self.sound);
        comp.insert("tick_chance", self.tick_chance);

        comp
    }
}

impl simdnbt::Serialize for Particle {
    fn to_compound(self) -> simdnbt::owned::NbtCompound {
        let mut comp = simdnbt::owned::NbtCompound::new();

        comp.insert("options", self.options.to_compound());
        comp.insert("probability", self.probability);

        comp
    }
}

impl simdnbt::Serialize for ParticleOptions {
    fn to_compound(self) -> simdnbt::owned::NbtCompound {
        let mut comp = simdnbt::owned::NbtCompound::new();

        comp.insert("type", self.particle_type);

        comp
    }
}
