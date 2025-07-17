#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnprefixedArray<T>(pub Vec<T>);

impl<T> From<Vec<T>> for UnprefixedArray<T> {
    fn from(vec: Vec<T>) -> Self {
        UnprefixedArray(vec)
    }
}

#[cfg(feature = "serde")]
use serde::{Serialize, ser::SerializeSeq};
#[cfg(feature = "serde")]
impl<T> Serialize for UnprefixedArray<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        // seq.serialize_element(&Varint::new(self.0.len() as i32))?;

        for item in &self.0 {
            seq.serialize_element(item)?;
        }
        seq.end()
    }
}
