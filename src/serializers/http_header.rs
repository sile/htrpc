use std::borrow::Cow;
use miasht::header::HeadersMut;
use serde::{ser, Serialize};
use serde::ser::Impossible;

use {Result, Error, ErrorKind};

/// `Serializer` implementation for HTTP header.
#[derive(Debug)]
pub struct HttpHeaderSerializer<'a> {
    is_first: bool,
    key: Option<Cow<'static, str>>,
    headers: HeadersMut<'a>,
}
impl<'a> HttpHeaderSerializer<'a> {
    /// Makes a new `HttpHeaderSerializer` instance.
    pub fn new(headers: HeadersMut<'a>) -> Self {
        HttpHeaderSerializer {
            is_first: true,
            key: None,
            headers,
        }
    }

    fn append(&mut self, key_or_val: Cow<str>) {
        if let Some(key) = self.key.take() {
            let val = &key_or_val;
            self.headers.add_raw_header(&key, val.as_bytes());
        } else {
            let key = key_or_val.into_owned();
            self.key = Some(Cow::Owned(key));
        }
    }
}
impl<'a, 'b> ser::Serializer for &'a mut HttpHeaderSerializer<'b> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        let s = if v { "ture" } else { "false" };
        self.append(Cow::Borrowed(s));
        Ok(())
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
        self.append(Cow::Owned(v.to_string()));
        Ok(())
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
        self.append(Cow::Owned(v.to_string()));
        Ok(())
    }
    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        track!(self.serialize_f64(v as f64))
    }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        self.append(Cow::Owned(v.to_string()));
        Ok(())
    }
    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        track!(self.serialize_str(&v.to_string()))
    }
    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        self.append(Cow::Borrowed(v));
        Ok(())
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        use std::str;
        let v = unsafe { str::from_utf8_unchecked(v) };
        self.append(Cow::Borrowed(v));
        Ok(())
    }
    fn serialize_none(self) -> Result<Self::Ok> {
        track_assert!(self.key.take().is_some(), ErrorKind::Invalid);
        Ok(())
    }
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
        where T: ?Sized + Serialize
    {
        track!(value.serialize(self))
    }
    fn serialize_unit(self) -> Result<Self::Ok> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        track!(self.serialize_str(name))
    }
    fn serialize_unit_variant(self,
                              _name: &'static str,
                              _variant_index: u32,
                              variant: &'static str)
                              -> Result<Self::Ok> {
        track!(self.serialize_str(variant))
    }
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
        where T: ?Sized + Serialize
    {
        track!(value.serialize(self))
    }
    fn serialize_newtype_variant<T>(self,
                                    _name: &'static str,
                                    _variant_index: u32,
                                    _variant: &'static str,
                                    value: &T)
                                    -> Result<Self::Ok>
        where T: ?Sized + Serialize
    {
        track!(value.serialize(self))
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_tuple_struct(self,
                              _name: &'static str,
                              _len: usize)
                              -> Result<Self::SerializeTupleStruct> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_tuple_variant(self,
                               _name: &'static str,
                               _variant_index: u32,
                               _variant: &'static str,
                               _len: usize)
                               -> Result<Self::SerializeTupleVariant> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        track_assert!(self.is_first, ErrorKind::Invalid);
        self.is_first = false;
        Ok(self)
    }
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        track_assert!(self.is_first, ErrorKind::Invalid);
        self.is_first = false;
        Ok(self)
    }
    fn serialize_struct_variant(self,
                                _name: &'static str,
                                _variant_index: u32,
                                _variant: &'static str,
                                _len: usize)
                                -> Result<Self::SerializeStructVariant> {
        track_assert!(self.is_first, ErrorKind::Invalid);
        self.is_first = false;
        Ok(self)
    }
}
impl<'a, 'b> ser::SerializeMap for &'a mut HttpHeaderSerializer<'b> {
    type Ok = ();
    type Error = Error;
    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        track_try!(key.serialize(&mut **self));
        Ok(())
    }
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        track_try!(value.serialize(&mut **self));
        Ok(())
    }
    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}
impl<'a, 'b> ser::SerializeStruct for &'a mut HttpHeaderSerializer<'b> {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        self.key = Some(Cow::Borrowed(key));
        track_try!(value.serialize(&mut **self));
        Ok(())
    }
    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}
impl<'a, 'b> ser::SerializeStructVariant for &'a mut HttpHeaderSerializer<'b> {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        self.key = Some(Cow::Borrowed(key));
        track_try!(value.serialize(&mut **self));
        Ok(())
    }
    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}
