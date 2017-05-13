use std;
use std::iter::Peekable;
use serde::de::{self, Visitor};
use miasht::header::{Headers, Iter as HeadersIter};
use trackable::error::IntoTrackableError;

use {Result, Error, ErrorKind};

#[derive(Debug, PartialEq, Eq)]
enum Phase {
    Key,
    Value,
}
impl Phase {
    pub fn is_key(&self) -> bool {
        *self == Phase::Key
    }
    pub fn next(&mut self) {
        if self.is_key() {
            *self = Phase::Value;
        } else {
            *self = Phase::Key;
        }
    }
}

/// `Deserializer` implementation for HTTP header.
#[derive(Debug)]
pub struct HttpHeaderDeserializer<'de> {
    in_map: bool,
    phase: Phase,
    headers: Peekable<HeadersIter<'de>>,
}
impl<'de> HttpHeaderDeserializer<'de> {
    /// Makes a new `HttpHeaderDeserializer` instance.
    pub fn new(headers: &'de Headers<'de>) -> Self {
        HttpHeaderDeserializer {
            in_map: false,
            phase: Phase::Key,
            headers: headers.iter().peekable(),
        }
    }
    fn is_end_of_header(&mut self) -> bool {
        self.headers.peek().is_none()
    }
    fn next_bytes(&mut self) -> Result<&'de [u8]> {
        if let Some(&(k, v)) = self.headers.peek() {
            let v = match self.phase {
                Phase::Key => k.as_bytes(),
                Phase::Value => {
                    let _ = self.headers.next();
                    v
                }
            };
            self.phase.next();
            Ok(v)
        } else {
            track_panic!(ErrorKind::Invalid);
        }
    }
}
impl<'de, 'a> de::Deserializer<'de> for &'a mut HttpHeaderDeserializer<'de> {
    type Error = Error;
    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Other, "unreachable");
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_bytes());
        let v = track_try!(parse_slice(v));
        track!(visitor.visit_bool(v))
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_bytes());
        let v = track_try!(parse_slice(v));
        track!(visitor.visit_i8(v))
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_bytes());
        let v = track_try!(parse_slice(v));
        track!(visitor.visit_i16(v))
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_bytes());
        let v = track_try!(parse_slice(v));
        track!(visitor.visit_i32(v))
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_bytes());
        let v = track_try!(parse_slice(v));
        track!(visitor.visit_i64(v))
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_bytes());
        let v = track_try!(parse_slice(v));
        track!(visitor.visit_u8(v))
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_bytes());
        let v = track_try!(parse_slice(v));
        track!(visitor.visit_u16(v))
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_bytes());
        let v = track_try!(parse_slice(v));
        track!(visitor.visit_u32(v))
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_bytes());
        let v = track_try!(parse_slice(v));
        track!(visitor.visit_u64(v))
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_bytes());
        let v = track_try!(parse_slice(v));
        track!(visitor.visit_f32(v))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_bytes());
        let v = track_try!(parse_slice(v));
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
        let v = track_try!(self.next_bytes());
        let v = track_try!(std::str::from_utf8(v));
        track!(visitor.visit_borrowed_str(v))
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track!(self.deserialize_str(visitor))
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_bytes());
        track!(visitor.visit_borrowed_bytes(v))
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track!(self.deserialize_bytes(visitor))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track!(visitor.visit_some(self))
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Invalid);
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Invalid);
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

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_assert!(!self.in_map, ErrorKind::Invalid);
        self.in_map = true;
        track!(visitor.visit_map(self))
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
        let _v = track_try!(self.next_bytes());
        track!(visitor.visit_unit()) // NOTE: dummy visiting
    }
}
impl<'de, 'a> de::MapAccess<'de> for &'a mut HttpHeaderDeserializer<'de> {
    type Error = Error;
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
        where K: de::DeserializeSeed<'de>
    {
        if self.is_end_of_header() {
            Ok(None)
        } else {
            let v = track_try!(seed.deserialize(&mut **self));
            Ok(Some(v))
        }
    }
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
        where V: de::DeserializeSeed<'de>
    {
        let v = track_try!(seed.deserialize(&mut **self));
        Ok(v)
    }
}

fn parse_slice<T: std::str::FromStr>(bytes: &[u8]) -> Result<T>
    where ErrorKind: IntoTrackableError<T::Err>
{
    let s = track_try!(std::str::from_utf8(bytes));
    let v = track_try!(s.parse());
    Ok(v)
}
