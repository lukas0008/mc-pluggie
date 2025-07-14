use std::{collections::VecDeque, error, fmt::Display};

use serde::de::{self, DeserializeOwned, DeserializeSeed, SeqAccess};

use crate::{Packet, varint::Varint};

pub trait DeserializePacket: Packet + DeserializeOwned {
    fn deserialize_packet(data: &[u8]) -> Result<Self, DeserializerError>;
}
impl<T> DeserializePacket for T
where
    T: Packet + DeserializeOwned,
{
    fn deserialize_packet(data: &[u8]) -> Result<T, DeserializerError> {
        let mut buf = VecDeque::from(data.to_vec());
        let serializer = Deserializer::new(&mut buf);

        T::deserialize(serializer)
    }
}

pub struct Deserializer<'a> {
    inner: &'a mut VecDeque<u8>,
}

#[derive(Debug)]
pub enum DeserializerError {
    Message(String),
    Stdio(std::io::Error),
}

impl Display for DeserializerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Message(msg) => f.write_str(msg),
            Self::Stdio(io) => io.fmt(f),
        }
    }
}

impl std::error::Error for DeserializerError {}

impl de::Error for DeserializerError {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Message(msg.to_string())
    }
}

impl<'a> Deserializer<'a> {
    pub fn new(buf: &'a mut VecDeque<u8>) -> Self {
        Self { inner: buf }
    }
}

impl<'a, 'de> de::Deserializer<'de> for Deserializer<'a> {
    type Error = DeserializerError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!(
            "This is impossible to do, since you cannot infer the data structure from the packet"
        )
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_bool(self.inner.pop_front() == Some(1))
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i8(i8::from_be_bytes([self.inner.pop_front().unwrap()]))
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i16(i16::from_be_bytes(
            self.inner
                .drain(0..2)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        ))
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i32(i32::from_be_bytes(
            self.inner
                .drain(0..4)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        ))
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i64(i64::from_be_bytes(
            self.inner
                .drain(0..8)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        ))
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u8(self.inner.pop_front().unwrap())
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u16(u16::from_be_bytes(
            self.inner
                .drain(0..2)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        ))
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u32(u32::from_be_bytes(
            self.inner
                .drain(0..4)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        ))
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u64(u64::from_be_bytes(
            self.inner
                .drain(0..8)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        ))
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f32(f32::from_be_bytes(
            self.inner
                .drain(0..4)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        ))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f64(f64::from_be_bytes(
            self.inner
                .drain(0..8)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        ))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let varint_range = self.inner.range(..5).map(Clone::clone).collect::<Vec<_>>();
        let (len, bytes_used) = Varint::parse(varint_range.as_slice()).unwrap();
        let _ = self.inner.drain(..bytes_used as usize);
        let string_range = self.inner.drain(..len.0 as usize).collect::<Vec<_>>();
        let string = String::from_utf8(string_range).unwrap();
        visitor.visit_str(&string)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let varint_range = self
            .inner
            .range(..5.min(self.inner.len()))
            .map(Clone::clone)
            .collect::<Vec<_>>();
        let (len, bytes_used) = Varint::parse(varint_range.as_slice()).unwrap();
        let _ = self.inner.drain(..bytes_used as usize);
        let string_range = self.inner.drain(..len.0 as usize).collect::<Vec<_>>();
        let string = String::from_utf8(string_range).unwrap();
        visitor.visit_string(string)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let present = self.inner.pop_front().unwrap_or(0);
        if present != 0 {
            visitor.visit_some(Deserializer { inner: self.inner })
        } else {
            visitor.visit_none()
        }
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        struct Access<'a, 'b> {
            deserializer: &'a mut Deserializer<'b>,
        }

        impl<'de, 'a, 'b: 'a> SeqAccess<'de> for Access<'a, 'b> {
            type Error = DeserializerError;

            fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
            where
                T: de::DeserializeSeed<'de>,
            {
                let value = DeserializeSeed::deserialize(
                    seed,
                    Deserializer {
                        inner: &mut self.deserializer.inner,
                    },
                )?;
                Ok(Some(value))
            }
        }

        let value = visitor.visit_seq(Access {
            deserializer: &mut self,
        });

        value
    }

    fn deserialize_tuple<V>(mut self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        struct Access<'a, 'b> {
            deserializer: &'a mut Deserializer<'b>,
            len: usize,
        }

        impl<'de, 'a, 'b: 'a> SeqAccess<'de> for Access<'a, 'b> {
            type Error = DeserializerError;

            fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
            where
                T: de::DeserializeSeed<'de>,
            {
                if self.len > 0 {
                    self.len -= 1;
                    let value = DeserializeSeed::deserialize(
                        seed,
                        Deserializer {
                            inner: &mut self.deserializer.inner,
                        },
                    )?;
                    Ok(Some(value))
                } else {
                    Ok(None)
                }
            }
        }

        let value = visitor.visit_seq(Access {
            deserializer: &mut self,
            len,
        });

        value
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }
}
