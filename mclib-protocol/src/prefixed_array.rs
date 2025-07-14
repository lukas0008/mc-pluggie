use std::marker::PhantomData;

use serde::{
    Deserialize, Serialize,
    de::{SeqAccess, Visitor},
    ser::SerializeSeq,
};

use crate::varint::Varint;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrefixedArray<T>(pub Vec<T>);

impl<T> From<Vec<T>> for PrefixedArray<T> {
    fn from(vec: Vec<T>) -> Self {
        PrefixedArray(vec)
    }
}

impl<'de, T> Deserialize<'de> for PrefixedArray<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PrefixedArrayVisitor<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for PrefixedArrayVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = PrefixedArray<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a varint")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let len = seq.next_element::<Varint>()?.unwrap().0;
                let mut vec = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    vec.push(seq.next_element()?.unwrap());
                }
                Ok(PrefixedArray(vec))
            }
        }
        deserializer.deserialize_seq(PrefixedArrayVisitor(PhantomData))
    }
}

impl<T> Serialize for PrefixedArray<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len() + 1))?;
        seq.serialize_element(&Varint::new(self.0.len() as i32))?;

        for item in &self.0 {
            seq.serialize_element(item)?;
        }
        seq.end()
    }
}
