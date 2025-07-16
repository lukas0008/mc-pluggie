// use std::{io::Read, marker::PhantomData};

// use serde::{
//     Serialize,
//     de::{Deserialize, DeserializeOwned, SeqAccess, Visitor},
// };

// pub struct NetworkNbt<T: Serialize + DeserializeOwned>(pub T);

// impl<T: Serialize + DeserializeOwned> Serialize for NetworkNbt<T> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let bytes = fastnbt::to_bytes(&self.0).map_err(|e| serde::ser::Error::custom(e))?;

//         serializer.serialize_bytes(&bytes)
//     }
// }

// impl<'de, T: Serialize + DeserializeOwned> Deserialize<'de> for NetworkNbt<T> {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         struct NbtVisitor<T>(PhantomData<T>);

//         impl<'de, T: Serialize + DeserializeOwned> Visitor<'de> for NbtVisitor<T> {
//             type Value = T;

//             fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//                 formatter.write_str("nbt")
//             }

//             fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
//             where
//                 A: SeqAccess<'de>,
//             {
//                 struct SeqReader<'de, T: SeqAccess<'de>> {
//                     seq: T,
//                     _marker: PhantomData<&'de ()>,
//                 }
//                 impl<'de, T: SeqAccess<'de>> Read for SeqReader<'de, T> {
//                     fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
//                         if let Ok(Some(v)) = self.seq.next_element::<u8>() {
//                             buf[0] = v;
//                             Ok(1)
//                         } else {
//                             Ok(0)
//                         }
//                     }
//                 }

//                 let reader = SeqReader {
//                     seq,
//                     _marker: PhantomData,
//                 };
//                 fastnbt::from_reader(reader).map_err(|e| serde::de::Error::custom(e))
//             }
//         }

//         deserializer.deserialize_seq(NbtVisitor(PhantomData))
//     }
// }
