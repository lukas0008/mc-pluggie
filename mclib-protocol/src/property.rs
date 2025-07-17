#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

impl Property {
    pub fn new(name: String, value: String, signature: Option<String>) -> Self {
        Property {
            name,
            value,
            signature,
        }
    }
}
