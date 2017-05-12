use fibers::net::TcpStream;
use miasht::server::Request;
use serde::de::{self, Visitor};
use url::Url;

use {Result, Error, ErrorKind};
use deserializers::{UrlPathDeserializer, UrlQueryDeserializer, HttpHeaderDeserializer,
                    HttpBodyDeserializer};
use path_template::PathTemplate;

// struct Request { path, query, header, body }

#[derive(Debug, Clone, Copy)]
enum Phase {
    Init,
    Path,
    Query,
    Header,
    Body,
}

#[derive(Debug)]
pub struct RequestDeserializer<'de> {
    phase: Phase,
    path_template: PathTemplate,
    url: &'de Url,
    request: &'de Request<TcpStream>,
    body: Vec<u8>,
}
impl<'de> RequestDeserializer<'de> {
    pub fn new(path_template: PathTemplate,
               url: &'de Url,
               request: &'de Request<TcpStream>,
               body: Vec<u8>)
               -> Self {
        RequestDeserializer {
            phase: Phase::Init,
            path_template,
            url,
            request,
            body,
        }
    }
}
impl<'de, 'a> de::Deserializer<'de> for &'a mut RequestDeserializer<'de> {
    type Error = Error;
    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Other, "unreachable");
    }
    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Invalid);
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
    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        track!(visitor.visit_unit()) // NOTE: dummy visiting
    }
}
impl<'de, 'a> de::MapAccess<'de> for &'a mut RequestDeserializer<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
        where K: de::DeserializeSeed<'de>
    {
        use serde::de::IntoDeserializer;
        use serde::de::value::StrDeserializer;

        match self.phase {
            Phase::Init => {
                self.phase = Phase::Path;
                let deserializer: StrDeserializer<Error> = "path".into_deserializer();
                let value = track_try!(seed.deserialize(deserializer));
                Ok(Some(value))
            }
            Phase::Path => {
                self.phase = Phase::Query;
                let deserializer: StrDeserializer<Error> = "query".into_deserializer();
                let value = track_try!(seed.deserialize(deserializer));
                Ok(Some(value))
            }
            Phase::Query => {
                self.phase = Phase::Header;
                let deserializer: StrDeserializer<Error> = "header".into_deserializer();
                let value = track_try!(seed.deserialize(deserializer));
                Ok(Some(value))
            }
            Phase::Header => {
                self.phase = Phase::Body;
                let deserializer: StrDeserializer<Error> = "body".into_deserializer();
                let value = track_try!(seed.deserialize(deserializer));
                Ok(Some(value))
            }
            Phase::Body => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
        where V: de::DeserializeSeed<'de>
    {
        match self.phase {
            Phase::Init => unreachable!(),
            Phase::Path => {
                let mut de = UrlPathDeserializer::new(self.path_template, self.url);
                let v = track_try!(seed.deserialize(&mut de));
                Ok(v)
            }
            Phase::Query => {
                let mut de = UrlQueryDeserializer::new(self.url.query_pairs());
                let v = track_try!(seed.deserialize(&mut de));
                Ok(v)
            }
            Phase::Header => {
                let mut de = HttpHeaderDeserializer::new(self.request.headers());
                let v = track_try!(seed.deserialize(&mut de));
                Ok(v)
            }
            Phase::Body => {
                use std::mem;
                let body = mem::replace(&mut self.body, Vec::new());
                let de = HttpBodyDeserializer::new(body);
                let v = track_try!(seed.deserialize(de));
                Ok(v)
            }
        }
    }
}
