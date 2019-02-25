#![allow(dead_code)]

// riot decided that some bools can be of garbage value with just the first bit being significant,
// this fucks us over if we were to just interpret the bytes as bools cause it seems that 0 is false
// and everything else is true in rust
pub(in crate) mod bit_bool {
    use serde::Deserialize;
    pub fn deserialize<'de, D>(d: D) -> Result<bool, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        u8::deserialize(d).map(|byte| byte & 1 != 0)
    }
    pub fn serialize<S>(b: &bool, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_u8(*b as u8)
    }
}

#[allow(clippy::cast_lossless)]
pub(in crate) mod f8 {
    use serde::Deserialize;
    pub fn deserialize<'de, D>(d: D) -> Result<f32, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        u8::deserialize(d).map(|byte| (byte as i32 - 128) as f32 / 100.0)
    }
    pub fn serialize<S>(float: &f32, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_u8(((float * 100.0) as i32 + 128) as u8)
    }
}

pub(in crate) mod lookahead_u8 {
    pub fn deserialize<'de, D, T: 'de>(d: D) -> Result<Option<T>, D::Error>
    where
        D: serde::Deserializer<'de>,
        T: serde::Deserialize<'de>,
    {
        use core::marker::PhantomData;
        use serde::de::{Error, SeqAccess, Visitor};

        struct OptVecVisitor<'de, T: serde::Deserialize<'de>>(PhantomData<&'de T>);

        impl<'de, T: serde::Deserialize<'de>> Visitor<'de> for OptVecVisitor<'de, T> {
            type Value = Option<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("opt")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let lookahead: u8 = seq
                    .next_element()?
                    .ok_or_else(|| Error::custom(crate::Error::UnexpectedEof))?;
                if lookahead != 0 {
                    seq.next_element()
                } else {
                    Ok(None)
                }
            }
        }

        d.deserialize_seq(OptVecVisitor(PhantomData))
    }
    pub fn serialize<S, T>(val: &Option<T>, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        T: serde::Serialize,
    {
        use serde::ser::SerializeTuple;
        if let Some(val) = val {
            let mut s = s.serialize_tuple(2)?;
            s.serialize_element(&1u8)?;
            s.serialize_element(val)?;
            s.end()
        } else {
            s.serialize_u8(0)
        }
    }
}

// for completeness sake
pub(in crate) mod string_null {
    use serde::de::Deserialize;
    #[inline]
    pub fn deserialize<'de, D>(d: D) -> Result<String, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(d)
    }
    #[inline]
    pub fn serialize<S>(string: &str, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(string)
    }
}

pub(in crate) mod sized_string {
    use serde::{
        de::{Error, SeqAccess, Visitor},
        ser::SerializeTuple,
    };
    pub fn deserialize<'de, D>(d: D) -> Result<String, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SizedStringVisitor;

        impl<'de> Visitor<'de> for SizedStringVisitor {
            type Value = String;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("seq")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let len: u32 = seq
                    .next_element()?
                    .ok_or_else(|| Error::custom(crate::Error::UnexpectedEof))?;
                let mut buf: Vec<u8> = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    buf.push(
                        seq.next_element()?
                            .ok_or_else(|| Error::custom(crate::Error::UnexpectedEof))?,
                    );
                }
                String::from_utf8(buf).map_err(|e| Error::custom(e.utf8_error()))
            }
        }

        d.deserialize_seq(SizedStringVisitor)
    }
    pub fn serialize<S>(string: &str, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = s.serialize_tuple(2)?;
        s.serialize_element(&(string.len() as u32))?;
        s.serialize_element(string.as_bytes())?;
        s.end()
    }
}

pub(in crate) mod sized_string_null {
    use serde::{
        de::{Error, SeqAccess, Visitor},
        ser::SerializeTuple,
    };
    pub fn deserialize<'de, D>(d: D) -> Result<String, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SizedStringNullVisitor;

        impl<'de> Visitor<'de> for SizedStringNullVisitor {
            type Value = String;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("seq")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let len: u32 = seq
                    .next_element()?
                    .ok_or_else(|| Error::custom(crate::Error::UnexpectedEof))?;
                let mut buf: Vec<u8> = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    buf.push(
                        seq.next_element()?
                            .ok_or_else(|| Error::custom(crate::Error::UnexpectedEof))?,
                    );
                }
                seq.next_element()?
                    .ok_or_else(|| Error::custom(crate::Error::UnexpectedEof))?;
                String::from_utf8(buf).map_err(|e| Error::custom(e.utf8_error()))
            }
        }

        d.deserialize_seq(SizedStringNullVisitor)
    }
    pub fn serialize<S>(string: &str, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = s.serialize_tuple(3)?;
        s.serialize_element(&(string.len() as u32))?;
        s.serialize_element(string.as_bytes())?;
        s.serialize_element(&0u8)?;
        s.end()
    }
}

pub(in crate) mod mask_0x7fff {
    use serde::Deserialize;
    pub fn deserialize<'de, D>(d: D) -> Result<u16, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        u16::deserialize(d).map(|short| short & 0x7fff)
    }
    pub fn serialize<S>(short: &u16, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_u16(short & 0x7fff)
    }
}

make_sized_vec!(vec_u8 u8);
make_sized_vec!(vec_u16 u16);
make_sized_vec!(vec_u32 u32);

make_fixed_string!(string_16 16);
make_fixed_string!(string_32 32);
make_fixed_string!(string_40 40);
make_fixed_string!(string_64 64);
make_fixed_string!(string_128 128);
make_fixed_string!(string_256 256);
