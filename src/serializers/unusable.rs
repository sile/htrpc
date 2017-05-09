use serde::{ser, Serialize};

use {Result, Error};

#[derive(Debug)]
pub struct UnusableSerializer {
    _cannot_instantiate: (),
}
impl ser::Serializer for UnusableSerializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok> {
        unreachable!();
    }
    fn serialize_i8(self, _v: i8) -> Result<Self::Ok> {
        unreachable!();
    }
    fn serialize_i16(self, _v: i16) -> Result<Self::Ok> {
        unreachable!();
    }
    fn serialize_i32(self, _v: i32) -> Result<Self::Ok> {
        unreachable!();
    }
    fn serialize_i64(self, _v: i64) -> Result<Self::Ok> {
        unreachable!();
    }
    fn serialize_u8(self, _v: u8) -> Result<Self::Ok> {
        unreachable!();
    }
    fn serialize_u16(self, _v: u16) -> Result<Self::Ok> {
        unreachable!();
    }
    fn serialize_u32(self, _v: u32) -> Result<Self::Ok> {
        unreachable!();
    }
    fn serialize_u64(self, _v: u64) -> Result<Self::Ok> {
        unreachable!();
    }
    fn serialize_f32(self, _v: f32) -> Result<Self::Ok> {
        unreachable!();
    }
    fn serialize_f64(self, _v: f64) -> Result<Self::Ok> {
        unreachable!();
    }
    fn serialize_char(self, _v: char) -> Result<Self::Ok> {
        unreachable!();
    }
    fn serialize_str(self, _v: &str) -> Result<Self::Ok> {
        unreachable!();
    }
    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok> {
        unreachable!();
    }
    fn serialize_none(self) -> Result<Self::Ok> {
        unreachable!();
    }
    fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok>
        where T: ?Sized + Serialize
    {
        unreachable!();
    }
    fn serialize_unit(self) -> Result<Self::Ok> {
        unreachable!();
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        unreachable!();
    }
    fn serialize_unit_variant(self,
                              _name: &'static str,
                              _variant_index: u32,
                              _variant: &'static str)
                              -> Result<Self::Ok> {
        unreachable!();
    }
    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<Self::Ok>
        where T: ?Sized + Serialize
    {
        unreachable!();
    }
    fn serialize_newtype_variant<T>(self,
                                    _name: &'static str,
                                    _variant_index: u32,
                                    _variant: &'static str,
                                    _value: &T)
                                    -> Result<Self::Ok>
        where T: ?Sized + Serialize
    {
        unreachable!();
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        unreachable!();
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        unreachable!();
    }
    fn serialize_tuple_struct(self,
                              _name: &'static str,
                              _len: usize)
                              -> Result<Self::SerializeTupleStruct> {
        unreachable!();
    }
    fn serialize_tuple_variant(self,
                               _name: &'static str,
                               _variant_index: u32,
                               _variant: &'static str,
                               _len: usize)
                               -> Result<Self::SerializeTupleVariant> {
        unreachable!();
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        unreachable!();
    }
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        unreachable!();
    }
    fn serialize_struct_variant(self,
                                _name: &'static str,
                                _variant_index: u32,
                                _variant: &'static str,
                                _len: usize)
                                -> Result<Self::SerializeStructVariant> {
        unreachable!();
    }
}
impl ser::SerializeTuple for UnusableSerializer {
    type Ok = ();
    type Error = Error;
    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        unreachable!();
    }
    fn end(self) -> Result<Self::Ok> {
        unreachable!();
    }
}
impl ser::SerializeTupleStruct for UnusableSerializer {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        unreachable!();
    }
    fn end(self) -> Result<Self::Ok> {
        unreachable!();
    }
}
impl ser::SerializeTupleVariant for UnusableSerializer {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        unreachable!();
    }
    fn end(self) -> Result<Self::Ok> {
        unreachable!();
    }
}
impl ser::SerializeSeq for UnusableSerializer {
    type Ok = ();
    type Error = Error;
    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        unreachable!();
    }
    fn end(self) -> Result<Self::Ok> {
        unreachable!();
    }
}
impl ser::SerializeMap for UnusableSerializer {
    type Ok = ();
    type Error = Error;
    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        unreachable!();
    }
    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        unreachable!();
    }
    fn end(self) -> Result<Self::Ok> {
        unreachable!();
    }
}
impl<'a> ser::SerializeStruct for UnusableSerializer {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        unreachable!();
    }
    fn end(self) -> Result<Self::Ok> {
        unreachable!();
    }
}
impl ser::SerializeStructVariant for UnusableSerializer {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        unreachable!();
    }
    fn end(self) -> Result<Self::Ok> {
        unreachable!();
    }
}
