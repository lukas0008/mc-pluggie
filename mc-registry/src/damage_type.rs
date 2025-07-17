#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct DamageType {
    pub exhaustion: f32,
    pub message_id: String,
    pub scaling: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub death_message_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effects: Option<String>,
}

impl simdnbt::Serialize for DamageType {
    fn to_compound(self) -> simdnbt::owned::NbtCompound {
        let mut comp = simdnbt::owned::NbtCompound::new();

        comp.insert("exhaustion", self.exhaustion);
        comp.insert("message_id", self.message_id);
        comp.insert("scaling", self.scaling);

        if let Some(death_message_type) = self.death_message_type {
            comp.insert("death_message_type", death_message_type);
        }

        if let Some(effects) = self.effects {
            comp.insert("effects", effects);
        }

        comp
    }
}
