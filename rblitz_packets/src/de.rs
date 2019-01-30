use byteorder::{ReadBytesExt, LE};
use serde::de::{self, Deserialize, DeserializeSeed, SeqAccess, Visitor};

use crate::error::{Error, Result};

#[allow(missing_copy_implementations, missing_debug_implementations)]
pub struct Deserializer<'de> {
    data: &'de [u8],
}

pub fn from_bytes<'a, T>(data: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer { data };
    T::deserialize(&mut deserializer)
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        panic!("unsupported")
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.data.read_u8()? != 0)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.data.read_i8()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.data.read_i16::<LE>()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.data.read_i32::<LE>()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.data.read_i64::<LE>()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.data.read_u8()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.data.read_u16::<LE>()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.data.read_u32::<LE>()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.data.read_u64::<LE>()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.data.read_f32::<LE>()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.data.read_f64::<LE>()?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_char(self.data.read_u8()? as char)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let end = self
            .data
            .iter()
            .position(|b| *b == 0)
            .ok_or(Error::UnexpectedEof)?;
        let slice = &self.data[..end];
        self.data = &self.data[end..];
        visitor.visit_str(std::str::from_utf8(slice)?)
    }

    // Deserializes a null-terminated string without passing the null byte
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let end = self
            .data
            .iter()
            .position(|b| *b == 0)
            .ok_or(Error::UnexpectedEof)?;
        let slice = &self.data[..end];
        self.data = &self.data[end..];
        visitor.visit_string(String::from_utf8(slice.to_owned())?)
    }

    fn deserialize_bytes<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        panic!("unsupported")
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(self, _: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(Seq { de: self })
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(FixedSeq { de: self, len })
    }

    fn deserialize_tuple_struct<V>(
        self,
        _: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        panic!("unsupported")
    }

    fn deserialize_struct<V>(
        self,
        _: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
        //visitor.visit_enum(self)
    }

    fn deserialize_identifier<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        panic!("unsupported")
    }

    fn deserialize_ignored_any<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        panic!("unsupported")
    }
}

struct FixedSeq<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    len: usize,
}

impl<'de, 'a> SeqAccess<'de> for FixedSeq<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.len == 0 {
            Ok(None)
        } else {
            seed.deserialize(&mut *self.de).map(Some)
        }
    }
}

struct Seq<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'de, 'a> SeqAccess<'de> for Seq<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.de.data.is_empty() {
            Ok(None)
        } else {
            seed.deserialize(&mut *self.de).map(Some)
        }
    }
}
