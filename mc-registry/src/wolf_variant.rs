#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct WolfVariant {
    pub assets: WolfAssets,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct WolfAssets {
    pub wild: String,
    pub tame: String,
    pub angry: String,
}

impl simdnbt::Serialize for WolfVariant {
    fn to_compound(self) -> simdnbt::owned::NbtCompound {
        let mut comp = simdnbt::owned::NbtCompound::new();

        let mut assets_comp = simdnbt::owned::NbtCompound::new();
        assets_comp.insert("wild", self.assets.wild);
        assets_comp.insert("tame", self.assets.tame);
        assets_comp.insert("angry", self.assets.angry);
        comp.insert("assets", assets_comp);

        comp
    }
}
