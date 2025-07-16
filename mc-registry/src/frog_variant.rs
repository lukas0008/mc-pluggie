#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct FrogVariant {
    pub asset_id: String,
}

impl simdnbt::Serialize for FrogVariant {
    fn to_compound(self) -> simdnbt::owned::NbtCompound {
        let mut comp = simdnbt::owned::NbtCompound::new();

        comp.insert("asset_id", self.asset_id);

        comp
    }
}
