use std;
use std::borrow::Cow;
use std::iter::Peekable;
use serde::de::{self, Visitor};
use trackable::error::IntoTrackableError;
use url;

use {Result, Error, ErrorKind};

#[derive(Debug, PartialEq, Eq)]
enum Phase<'a> {
    Key,
    Value(Cow<'a, str>),
}
impl<'a> Phase<'a> {
    pub fn take(&mut self) -> Self {
        std::mem::replace(self, Phase::Key)
    }
}

/// `Deserializer` implementation for URL query string.
pub struct UrlQueryDeserializer<'de> {
    in_map: bool,
    phase: Phase<'de>,
    query: Peekable<url::form_urlencoded::Parse<'de>>,
}
impl<'de> UrlQueryDeserializer<'de> {
    /// Makes a new `UrlQueryDeserializer` instance.
    pub fn new(query: url::form_urlencoded::Parse<'de>) -> Self {
        UrlQueryDeserializer {
            in_map: false,
            phase: Phase::Key,
            query: query.peekable(),
        }
    }

    fn is_end_of_query(&mut self) -> bool {
        self.query.peek().is_none()
    }
    fn next_str(&mut self) -> Result<Cow<'de, str>> {
        match self.phase.take() {
            Phase::Key => {
                let (k, v) = track_try!(self.query.next().ok_or(ErrorKind::Invalid));
                self.phase = Phase::Value(v);
                Ok(k)
            }
            Phase::Value(v) => Ok(v),
        }
    }
}
impl<'de, 'a> de::Deserializer<'de> for &'a mut UrlQueryDeserializer<'de> {
    type Error = Error;
    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Other, "unreachable");
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_str());
        let v = track_try!(parse_cow_str(v));
        track!(visitor.visit_bool(v))
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_str());
        let v = track_try!(parse_cow_str(v));
        track!(visitor.visit_i8(v))
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_str());
        let v = track_try!(parse_cow_str(v));
        track!(visitor.visit_i16(v))
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_str());
        let v = track_try!(parse_cow_str(v));
        track!(visitor.visit_i32(v))
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_str());
        let v = track_try!(parse_cow_str(v));
        track!(visitor.visit_i64(v))
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_str());
        let v = track_try!(parse_cow_str(v));
        track!(visitor.visit_u8(v))
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_str());
        let v = track_try!(parse_cow_str(v));
        track!(visitor.visit_u16(v))
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_str());
        let v = track_try!(parse_cow_str(v));
        track!(visitor.visit_u32(v))
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_str());
        let v = track_try!(parse_cow_str(v));
        track!(visitor.visit_u64(v))
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_str());
        let v = track_try!(parse_cow_str(v));
        track!(visitor.visit_f32(v))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_str());
        let v = track_try!(parse_cow_str(v));
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
        let v = track_try!(self.next_str());
        match v {
            Cow::Borrowed(s) => track!(visitor.visit_borrowed_str(s)),
            Cow::Owned(s) => track!(visitor.visit_string(s)),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track!(self.deserialize_str(visitor))
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_str());
        match v {
            Cow::Borrowed(s) => track!(visitor.visit_borrowed_bytes(s.as_bytes())),
            Cow::Owned(s) => track!(visitor.visit_byte_buf(s.into_bytes())),
        }
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

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_str());
        track_assert!(v.is_empty(), ErrorKind::Invalid);
        track!(visitor.visit_unit())
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track!(self.deserialize_unit(visitor))
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

    forward_to_deserialize_any! {
        ignored_any
    }
}
impl<'de, 'a> de::MapAccess<'de> for &'a mut UrlQueryDeserializer<'de> {
    type Error = Error;
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
        where K: de::DeserializeSeed<'de>
    {
        if self.is_end_of_query() {
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

fn parse_cow_str<T: std::str::FromStr>(s: Cow<str>) -> Result<T>
    where ErrorKind: IntoTrackableError<T::Err>
{
    let v = track_try!(s.parse(), "s={:?}", s);
    Ok(v)
}

#[cfg(test)]
mod test {
    use serde::Deserialize;
    use url::Url;
    use super::*;

    #[test]
    fn struct_works() {
        #[derive(Deserialize)]
        struct Params {
            foo: Option<usize>,
            bar: String,
        }

        let url = Url::parse("http://localhost/?bar=baz+qux").unwrap();
        {
            let mut deserializer = UrlQueryDeserializer::new(url.query_pairs());
            let params = track_try_unwrap!(Params::deserialize(&mut deserializer));
            assert_eq!(params.foo, None);
            assert_eq!(params.bar, "baz qux");
        }

        let url = Url::parse("http://localhost/?foo=10&bar=baz+qux").unwrap();
        {
            let mut deserializer = UrlQueryDeserializer::new(url.query_pairs());
            let params = track_try_unwrap!(Params::deserialize(&mut deserializer));
            assert_eq!(params.foo, Some(10));
            assert_eq!(params.bar, "baz qux");
        }
    }
}
