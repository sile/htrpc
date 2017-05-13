use std;
use serde::de::{self, Visitor};
use trackable::error::IntoTrackableError;

use {Result, Error, ErrorKind};

/// `Deserializer` implementation for HTTP body.
#[derive(Debug)]
pub struct HttpBodyDeserializer {
    body: Vec<u8>,
}
impl HttpBodyDeserializer {
    /// Makes a new `HttpBodyDeserializer` instance.
    pub fn new(body: Vec<u8>) -> Self {
        HttpBodyDeserializer { body }
    }
}
impl<'de> de::Deserializer<'de> for HttpBodyDeserializer {
    type Error = Error;
    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Other, "unreachable");
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(parse_slice(&self.body[..]));
        track!(visitor.visit_bool(v))
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(parse_slice(&self.body[..]));
        track!(visitor.visit_i8(v))
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(parse_slice(&self.body[..]));
        track!(visitor.visit_i16(v))
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(parse_slice(&self.body[..]));
        track!(visitor.visit_i32(v))
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(parse_slice(&self.body[..]));
        track!(visitor.visit_i64(v))
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(parse_slice(&self.body[..]));
        track!(visitor.visit_u8(v))
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(parse_slice(&self.body[..]));
        track!(visitor.visit_u16(v))
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(parse_slice(&self.body[..]));
        track!(visitor.visit_u32(v))
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(parse_slice(&self.body[..]));
        track!(visitor.visit_u64(v))
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(parse_slice(&self.body[..]));
        track!(visitor.visit_f32(v))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(parse_slice(&self.body[..]));
        track!(visitor.visit_f64(v))
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Invalid, "Unsupported");
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track!(self.deserialize_string(visitor))
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(String::from_utf8(self.body));
        track!(visitor.visit_string(v))
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track!(self.deserialize_byte_buf(visitor))
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track!(visitor.visit_byte_buf(self.body))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track!(visitor.visit_some(self))
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track!(visitor.visit_unit())
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track!(visitor.visit_unit())
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Invalid);
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Invalid);
    }

    fn deserialize_tuple_struct<V>(self,
                                   _name: &'static str,
                                   _len: usize,
                                   _visitor: V)
                                   -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Invalid);
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Invalid);
    }

    fn deserialize_struct<V>(self,
                             _name: &'static str,
                             _fields: &'static [&'static str],
                             visitor: V)
                             -> Result<V::Value>
        where V: Visitor<'de>
    {
        track!(self.deserialize_map(visitor))
    }

    fn deserialize_enum<V>(self,
                           _name: &'static str,
                           _variants: &'static [&'static str],
                           _visitor: V)
                           -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Invalid);
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track!(self.deserialize_str(visitor))
    }
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track!(visitor.visit_unit()) // NOTE: dummy visiting
    }
}

fn parse_slice<T: std::str::FromStr>(bytes: &[u8]) -> Result<T>
    where ErrorKind: IntoTrackableError<T::Err>
{
    let s = track_try!(std::str::from_utf8(bytes));
    let v = track_try!(s.parse());
    Ok(v)
}
