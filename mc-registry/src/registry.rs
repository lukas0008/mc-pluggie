use crate::cat_variant::CatVariant;
use crate::chicken_variant::ChickenVariant;
use crate::cow_variant::CowVariant;
use crate::dimension_type::DimensionType;
use crate::frog_variant::FrogVariant;
use crate::painting_variant::PaintingVariant;
use crate::pig_variant::PigVariant;
use crate::wolf_sound_variant::WolfSoundVariant;
use crate::wolf_variant::WolfVariant;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Registry {
    #[serde(rename = "dimension_type")]
    #[serde(with = "tuple_vec_map")]
    pub dimension_types: Vec<(String, DimensionType)>,
    #[serde(rename = "chicken_variant")]
    #[serde(with = "tuple_vec_map")]
    pub chicken_variants: Vec<(String, ChickenVariant)>,
    #[serde(rename = "cow_variant")]
    #[serde(with = "tuple_vec_map")]
    pub cow_variants: Vec<(String, CowVariant)>,
    #[serde(rename = "frog_variant")]
    #[serde(with = "tuple_vec_map")]
    pub frog_variants: Vec<(String, FrogVariant)>,
    #[serde(rename = "painting_variant")]
    #[serde(with = "tuple_vec_map")]
    pub painting_variants: Vec<(String, PaintingVariant)>,
    #[serde(rename = "pig_variant")]
    #[serde(with = "tuple_vec_map")]
    pub pig_variants: Vec<(String, PigVariant)>,
    #[serde(rename = "wolf_sound_variant")]
    #[serde(with = "tuple_vec_map")]
    pub wolf_sound_variants: Vec<(String, WolfSoundVariant)>,
    #[serde(rename = "wolf_variant")]
    #[serde(with = "tuple_vec_map")]
    pub wolf_variants: Vec<(String, WolfVariant)>,
    #[serde(rename = "cat_variant")]
    #[serde(with = "tuple_vec_map")]
    pub cat_variants: Vec<(String, CatVariant)>,
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            dimension_types: Vec::new(),
            chicken_variants: Vec::new(),
            cow_variants: Vec::new(),
            frog_variants: Vec::new(),
            painting_variants: Vec::new(),
            pig_variants: Vec::new(),
            wolf_sound_variants: Vec::new(),
            wolf_variants: Vec::new(),
            cat_variants: Vec::new(),
        }
    }
    pub fn extend(&mut self, other: Registry) {
        self.dimension_types.extend(other.dimension_types);
        self.chicken_variants.extend(other.chicken_variants);
        self.cow_variants.extend(other.cow_variants);
        self.frog_variants.extend(other.frog_variants);
        self.painting_variants.extend(other.painting_variants);
        self.pig_variants.extend(other.pig_variants);
        self.wolf_sound_variants.extend(other.wolf_sound_variants);
        self.wolf_variants.extend(other.wolf_variants);
        self.cat_variants.extend(other.cat_variants);
    }
}
