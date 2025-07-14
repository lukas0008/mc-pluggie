use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{SeqAccess, Visitor},
};

#[derive(Debug)]
pub struct Varint(pub i32);

impl From<i32> for Varint {
    fn from(value: i32) -> Self {
        Varint(value)
    }
}

impl Varint {
    pub fn new(value: i32) -> Self {
        Varint(value)
    }

    /// Parse a variable-length integer from a byte slice.
    /// Returns the parsed integer and the number of bytes read.
    pub fn parse(bytes: &[u8]) -> Option<(Self, u8)> {
        const CONTINUE: u8 = 0b10000000;
        const MASK: u8 = 0b01111111;

        let mut value = 0i32;

        for i in 0..5 {
            let byte = bytes.get(i)?;
            value |= ((byte & MASK) as i32) << (i * 7);
            if byte & CONTINUE == 0 {
                return Some((Self(value), i as u8 + 1));
            }
        }

        None
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut value = self.0;
        let mut bytes = Vec::new();

        for _ in 0..5 {
            const CONTINUE: u8 = 0b10000000;
            const MASK: u8 = 0b01111111;

            let byte = (value & MASK as i32) as u8;

            value >>= 7;
            if value > 0 {
                bytes.push(byte | CONTINUE);
            } else {
                bytes.push(byte);
                break;
            }

            if value == 0 {
                break;
            }
        }

        bytes
    }
}

impl Serialize for Varint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut value = self.0;

        let mut bytes = Vec::new();

        for _ in 0..5 {
            const CONTINUE: u8 = 0b10000000;
            const MASK: u8 = 0b01111111;

            let mut byte = (value & MASK as i32) as u8;

            value >>= 7;
            if value > 0 {
                byte |= CONTINUE;
            }
            bytes.push(byte);

            if value == 0 {
                break;
            }
        }

        serializer.serialize_bytes(&bytes)
    }
}

impl<'de> Deserialize<'de> for Varint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VarintVisitor;

        impl<'de> Visitor<'de> for VarintVisitor {
            type Value = Varint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a varint")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut val = 0;
                for i in 0..5 {
                    if let Some(byte) = seq.next_element::<u8>()? {
                        val |= (i32::from(byte) & 0b01111111) << (i * 7);
                        if byte & 0b10000000 == 0 {
                            return Ok(Varint(val));
                        }
                    } else {
                        break;
                    }
                }
                Err(serde::de::Error::custom("Varint too large"))
            }
        }

        deserializer.deserialize_seq(VarintVisitor)
    }
}
