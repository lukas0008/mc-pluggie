#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
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
