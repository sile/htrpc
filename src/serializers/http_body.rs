use serde::Serialize;
use serde::ser::{self, Impossible};

use {Result, Error, ErrorKind};

/// `Serializer` implementation for HTTP body.
#[derive(Debug)]
pub struct HttpBodySerializer;
impl ser::Serializer for HttpBodySerializer {
    type Ok = Vec<u8>;
    type Error = Error;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        let s = if v { "ture" } else { "false" };
        Ok(Vec::from(s.as_bytes()))
    }
    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        track!(self.serialize_i64(v as i64))
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        track!(self.serialize_i64(v as i64))
    }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        track!(self.serialize_i64(v as i64))
    }
    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        Ok(v.to_string().into_bytes())
    }
    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        track!(self.serialize_u64(v as u64))
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        track!(self.serialize_u64(v as u64))
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        track!(self.serialize_u64(v as u64))
    }
    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        Ok(v.to_string().into_bytes())
    }
    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        track!(self.serialize_f64(v as f64))
    }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        Ok(v.to_string().into_bytes())
    }
    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        track!(self.serialize_str(&v.to_string()))
    }
    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        Ok(v.to_string().into_bytes())
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        Ok(Vec::from(v))
    }
    fn serialize_none(self) -> Result<Self::Ok> {
        Ok(Vec::new())
    }
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        track!(value.serialize(self))
    }
    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(Vec::new())
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        Ok(Vec::new())
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        track!(self.serialize_str(variant))
    }
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        track!(value.serialize(self))
    }
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        track!(value.serialize(self))
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        track_panic!(ErrorKind::Invalid);
    }
}
