#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct WolfSoundVariant {
    pub hurt_sound: String,
    pub pant_sound: String,
    pub whine_sound: String,
    pub ambient_sound: String,
    pub death_sound: String,
    pub growl_sound: String,
}

impl simdnbt::Serialize for WolfSoundVariant {
    fn to_compound(self) -> simdnbt::owned::NbtCompound {
        let mut comp = simdnbt::owned::NbtCompound::new();

        comp.insert("hurt_sound", self.hurt_sound);
        comp.insert("pant_sound", self.pant_sound);
        comp.insert("whine_sound", self.whine_sound);
        comp.insert("ambient_sound", self.ambient_sound);
        comp.insert("death_sound", self.death_sound);
        comp.insert("growl_sound", self.growl_sound);

        comp
    }
}
