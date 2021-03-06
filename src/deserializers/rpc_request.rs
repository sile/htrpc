use fibers::net::TcpStream;
use miasht::server::Request;
use serde::de::{self, Visitor};
use url::Url;

use {Error, ErrorKind, Result};
use deserializers::{HttpHeaderDeserializer, UrlPathDeserializer, UrlQueryDeserializer};
use types::EntryPoint;

#[derive(Debug, Clone, Copy)]
enum Phase {
    Init,
    Path,
    Query,
    Header,
}

/// `Deserializer` implementation for RPC request.
#[derive(Debug)]
pub struct RpcRequestDeserializer<'de> {
    phase: Phase,
    entry_point: EntryPoint,
    url: &'de Url,
    request: &'de Request<TcpStream>,
}
impl<'de> RpcRequestDeserializer<'de> {
    /// Makes a new `RpcRequestDeserializer` instance.
    pub fn new(entry_point: EntryPoint, url: &'de Url, request: &'de Request<TcpStream>) -> Self {
        RpcRequestDeserializer {
            phase: Phase::Init,
            entry_point,
            url,
            request,
        }
    }
}
impl<'de, 'a> de::Deserializer<'de> for &'a mut RpcRequestDeserializer<'de> {
    type Error = Error;
    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track_panic!(ErrorKind::Other, "unreachable");
    }
    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }
    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track!(visitor.visit_map(self))
    }
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track!(self.deserialize_map(visitor))
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
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track!(visitor.visit_unit()) // NOTE: dummy visiting
    }
}
impl<'de, 'a> de::MapAccess<'de> for &'a mut RpcRequestDeserializer<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        use serde::de::IntoDeserializer;
        use serde::de::value::StrDeserializer;

        match self.phase {
            Phase::Init => {
                self.phase = Phase::Path;
                let deserializer: StrDeserializer<Error> = "path".into_deserializer();
                let value = track!(seed.deserialize(deserializer))?;
                Ok(Some(value))
            }
            Phase::Path => {
                self.phase = Phase::Query;
                let deserializer: StrDeserializer<Error> = "query".into_deserializer();
                let value = track!(seed.deserialize(deserializer))?;
                Ok(Some(value))
            }
            Phase::Query => {
                self.phase = Phase::Header;
                let deserializer: StrDeserializer<Error> = "header".into_deserializer();
                let value = track!(seed.deserialize(deserializer))?;
                Ok(Some(value))
            }
            Phase::Header => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        match self.phase {
            Phase::Init => unreachable!(),
            Phase::Path => {
                let mut de = track!(UrlPathDeserializer::new(self.entry_point, self.url))?;
                let v = track!(seed.deserialize(&mut de))?;
                Ok(v)
            }
            Phase::Query => {
                let mut de = UrlQueryDeserializer::new(self.url.query_pairs());
                let v = track!(seed.deserialize(&mut de))?;
                Ok(v)
            }
            Phase::Header => {
                let mut de = HttpHeaderDeserializer::new(self.request.headers());
                let v = track!(seed.deserialize(&mut de))?;
                Ok(v)
            }
        }
    }
}
