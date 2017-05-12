use std;
use std::borrow::Cow;
use std::iter::Peekable;
use std::str::Split;
use serde::de::{self, Visitor};
use trackable::error::IntoTrackableError;
use url::{self, Url};

use {Result, Error, ErrorKind, EntryPoint};

#[derive(Debug)]
pub struct UrlPathDeserializer<'de> {
    in_seq: bool,
    segments: Peekable<Split<'de, char>>,
    template: EntryPoint,
    index: usize,
}
impl<'de> UrlPathDeserializer<'de> {
    pub fn new(template: EntryPoint, url: &'de Url) -> Self {
        UrlPathDeserializer {
            in_seq: false,
            segments: url.path_segments().expect("TODO").peekable(),
            template,
            index: 0,
        }
    }
    fn is_end_of_segment(&mut self) -> bool {
        self.segments.peek().is_none()
    }
    fn is_end_of_parameter(&mut self) -> bool {
        // TODO: optimize
        !self.template.is_var_remaning(self.index)
    }

    fn finish(&mut self) -> Result<()> {
        for i in self.index..self.template.len() {
            let expected = track_try!(self.template.get_val(i).ok_or(ErrorKind::Invalid));
            let actual = track_try!(self.segments.next().ok_or(ErrorKind::Invalid));
            track_assert_eq!(actual, expected, ErrorKind::Invalid);
        }
        Ok(())
    }
    fn next_value(&mut self) -> Result<&'de str> {
        track_assert!(self.index < self.template.len(), ErrorKind::Invalid);
        track_assert!(!self.is_end_of_segment(), ErrorKind::Invalid);
        let i = self.index;
        self.index += 1;
        if let Some(expected) = self.template.get_val(i) {
            let s = self.segments.next().unwrap();
            track_assert_eq!(s, expected, ErrorKind::Invalid);
            self.next_value()
        } else {
            let s = self.segments.next().unwrap();
            Ok(s)
        }
    }
}
impl<'de, 'a> de::Deserializer<'de> for &'a mut UrlPathDeserializer<'de> {
    type Error = Error;
    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Other, "unreachable");
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_value());
        let v = track_try!(parse_escaped_str(v));
        track!(visitor.visit_bool(v))
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_value());
        let v = track_try!(parse_escaped_str(v));
        track!(visitor.visit_i8(v))
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_value());
        let v = track_try!(parse_escaped_str(v));
        track!(visitor.visit_i16(v))
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_value());
        let v = track_try!(parse_escaped_str(v));
        track!(visitor.visit_i32(v))
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_value());
        let v = track_try!(parse_escaped_str(v));
        track!(visitor.visit_i64(v))
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_value());
        let v = track_try!(parse_escaped_str(v));
        track!(visitor.visit_u8(v))
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_value());
        let v = track_try!(parse_escaped_str(v));
        track!(visitor.visit_u16(v))
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_value());
        let v = track_try!(parse_escaped_str(v));
        track!(visitor.visit_u32(v))
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_value());
        let v = track_try!(parse_escaped_str(v));
        track!(visitor.visit_u64(v))
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_value());
        let v = track_try!(parse_escaped_str(v));
        track!(visitor.visit_f32(v))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let v = track_try!(self.next_value());
        let v = track_try!(parse_escaped_str(v));
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
        let v = track_try!(self.next_value());
        let v = track_try!(url::percent_encoding::percent_decode(v.as_bytes()).decode_utf8());
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

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Invalid);
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track!(self.deserialize_bytes(visitor))
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Invalid);
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Invalid);
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

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_assert!(!self.in_seq, ErrorKind::Invalid);
        self.in_seq = true;
        track!(visitor.visit_seq(self))
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track!(self.deserialize_seq(visitor))
    }

    fn deserialize_tuple_struct<V>(self,
                                   _name: &'static str,
                                   _len: usize,
                                   visitor: V)
                                   -> Result<V::Value>
        where V: Visitor<'de>
    {
        track!(self.deserialize_seq(visitor))
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

    forward_to_deserialize_any! {
        ignored_any
    }
}
impl<'de, 'a> de::SeqAccess<'de> for &'a mut UrlPathDeserializer<'de> {
    type Error = Error;
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
        where T: de::DeserializeSeed<'de>
    {
        if self.is_end_of_parameter() {
            track_try!(self.finish());
            Ok(None)
        } else {
            let v = track_try!(seed.deserialize(&mut **self));
            Ok(Some(v))
        }
    }
}

fn parse_escaped_str<T: std::str::FromStr>(s: &str) -> Result<T>
    where ErrorKind: IntoTrackableError<T::Err>
{
    let s = track_try!(url::percent_encoding::percent_decode(s.as_bytes()).decode_utf8());
    let v = track_try!(s.parse(), "s={:?}", s);
    Ok(v)
}

#[cfg(test)]
mod test {
    use serde::Deserialize;
    use url::Url;

    use super::*;

    #[test]
    fn it_works() {
        let entry_point = htrpc_entry_point!["foo", _, "baz", _];

        #[derive(Deserialize)]
        struct Args(String, usize);

        let url = Url::parse("http://localhost/foo/hello%20world/baz/3").unwrap();
        let mut deserializer = UrlPathDeserializer::new(entry_point, &url);
        let Args(v0, v1) = track_try_unwrap!(Args::deserialize(&mut deserializer));
        assert_eq!(v0, "hello world");
        assert_eq!(v1, 3);
    }
}
