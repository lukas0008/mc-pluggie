use crate::dimension_type::DimensionType;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Registry {
    #[serde(rename = "dimension_type")]
    #[serde(with = "tuple_vec_map")]
    pub dimension_types: Vec<(String, DimensionType)>,
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            dimension_types: Vec::new(),
        }
    }
    pub fn extend(&mut self, other: Registry) {
        self.dimension_types.extend(other.dimension_types);
    }
}
