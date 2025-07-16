#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct PigVariant {
    pub model: Option<String>,
    pub asset_id: String,
}

impl simdnbt::Serialize for PigVariant {
    fn to_compound(self) -> simdnbt::owned::NbtCompound {
        let mut comp = simdnbt::owned::NbtCompound::new();

        if let Some(model) = self.model {
            comp.insert("model", model);
        }
        comp.insert("asset_id", self.asset_id);

        comp
    }
}
