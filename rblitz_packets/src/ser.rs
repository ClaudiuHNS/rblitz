use byteorder::{WriteBytesExt, LE};
use serde::ser::{self, Serialize};

use std::io::Write;

use crate::error::{Error, Result};

#[allow(missing_copy_implementations, missing_debug_implementations)]
pub struct Serializer<W: Write> {
    output: W,
}

pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut serializer = Serializer {
        output: Vec::<u8>::new(),
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

pub fn to_writer<T, W>(value: &T, w: W) -> Result<()>
where
    T: Serialize,
    W: Write,
{
    value.serialize(&mut Serializer { output: w })
}

impl<'a, W: Write> ser::Serializer for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.output
            .write_u8(v as u8)
            .map_err(|_| Error::UnexpectedEof)
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.output.write_i8(v).map_err(|_| Error::UnexpectedEof)
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.output
            .write_i16::<LE>(v)
            .map_err(|_| Error::UnexpectedEof)
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.output
            .write_i32::<LE>(v)
            .map_err(|_| Error::UnexpectedEof)
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.output
            .write_i64::<LE>(v)
            .map_err(|_| Error::UnexpectedEof)
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.output.write_u8(v).map_err(|_| Error::UnexpectedEof)
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.output
            .write_u16::<LE>(v)
            .map_err(|_| Error::UnexpectedEof)
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.output
            .write_u32::<LE>(v)
            .map_err(|_| Error::UnexpectedEof)
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.output
            .write_u64::<LE>(v)
            .map_err(|_| Error::UnexpectedEof)
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.output
            .write_f32::<LE>(v)
            .map_err(|_| Error::UnexpectedEof)
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.output
            .write_f64::<LE>(v)
            .map_err(|_| Error::UnexpectedEof)
    }

    fn serialize_char(self, _: char) -> Result<()> {
        unimplemented!()
    }

    fn serialize_str(self, s: &str) -> Result<()> {
        self.output.write_all(s.as_bytes())?;
        self.output.write_u8(0).map_err(|_| Error::UnexpectedEof)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.output.write_all(v).map_err(|_| Error::UnexpectedEof)
    }

    fn serialize_none(self) -> Result<()> {
        panic!("unsupported")
    }

    fn serialize_some<T>(self, _: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        panic!("unsupported")
    }

    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<()> {
        panic!("unsupported")
    }

    fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        panic!("unsupported")
    }

    fn serialize_newtype_variant<T>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        panic!("unsupported")
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(self)
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        panic!("unsupported")
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        panic!("unsupported")
    }

    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(self)
    }
}

impl<'a, W: Write> ser::SerializeSeq for &'a mut Serializer<W> {
    type Ok = <&'a mut Serializer<W> as ser::Serializer>::Ok;
    type Error = <&'a mut Serializer<W> as ser::Serializer>::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeTuple for &'a mut Serializer<W> {
    type Ok = <&'a mut Serializer<W> as ser::Serializer>::Ok;
    type Error = <&'a mut Serializer<W> as ser::Serializer>::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeTupleStruct for &'a mut Serializer<W> {
    type Ok = <&'a mut Serializer<W> as ser::Serializer>::Ok;
    type Error = <&'a mut Serializer<W> as ser::Serializer>::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeTupleVariant for &'a mut Serializer<W> {
    type Ok = <&'a mut Serializer<W> as ser::Serializer>::Ok;
    type Error = <&'a mut Serializer<W> as ser::Serializer>::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeMap for &'a mut Serializer<W> {
    type Ok = <&'a mut Serializer<W> as ser::Serializer>::Ok;
    type Error = <&'a mut Serializer<W> as ser::Serializer>::Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeStruct for &'a mut Serializer<W> {
    type Ok = <&'a mut Serializer<W> as ser::Serializer>::Ok;
    type Error = <&'a mut Serializer<W> as ser::Serializer>::Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeStructVariant for &'a mut Serializer<W> {
    type Ok = <&'a mut Serializer<W> as ser::Serializer>::Ok;
    type Error = <&'a mut Serializer<W> as ser::Serializer>::Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}
