use fibers::net::TcpStream;
use miasht::client::Response;
use serde::de::{self, Visitor, IntoDeserializer};

use {Result, Error, ErrorKind};
use deserializers::{HttpHeaderDeserializer, HttpBodyDeserializer};

#[derive(Debug, Clone, Copy)]
enum Phase {
    Init,
    Status,
    Header,
    Body,
}

/// `Deserializer` implementation for RPC response.
#[derive(Debug)]
pub struct RpcResponseDeserializer<'de> {
    phase: Phase,
    response: &'de Response<TcpStream>,
    body: Vec<u8>,
}
impl<'de> RpcResponseDeserializer<'de> {
    /// Makes a new `RpcResponseDeserializer` instance.
    pub fn new(response: &'de Response<TcpStream>, body: Vec<u8>) -> Self {
        RpcResponseDeserializer {
            phase: Phase::Init,
            response,
            body,
        }
    }
}
impl<'de, 'a> de::Deserializer<'de> for &'a mut RpcResponseDeserializer<'de> {
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
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track!(visitor.visit_enum(Enum(self)))
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
impl<'de, 'a> de::MapAccess<'de> for &'a mut RpcResponseDeserializer<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        use serde::de::value::StrDeserializer;
        match self.phase {
            Phase::Init => {
                self.phase = Phase::Status;
                let deserializer: StrDeserializer<Error> = "status".into_deserializer();
                let value = track_try!(seed.deserialize(deserializer));
                Ok(Some(value))
            }
            Phase::Status => {
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
    where
        V: de::DeserializeSeed<'de>,
    {
        use serde::de::value::U16Deserializer;
        match self.phase {
            Phase::Init => unreachable!(),
            Phase::Status => {
                let de: U16Deserializer<Error> = self.response.status().code().into_deserializer();
                let v = track_try!(seed.deserialize(de));
                Ok(v)
            }
            Phase::Header => {
                let mut de = HttpHeaderDeserializer::new(self.response.headers());
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

struct Enum<'de: 'a, 'a>(&'a mut RpcResponseDeserializer<'de>);
impl<'de, 'a> de::EnumAccess<'de> for Enum<'de, 'a> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'de>,
    {
        use serde::de::IntoDeserializer;
        use serde::de::value::StrDeserializer;
        let val = {
            let status = track_try!(status_code_to_str(self.0.response.status().code()));
            self.0.phase = Phase::Status;
            let deserializer: StrDeserializer<Error> = status.into_deserializer();
            track_try!(seed.deserialize(deserializer))
        };
        Ok((val, self))
    }
}
impl<'de, 'a> de::VariantAccess<'de> for Enum<'de, 'a> {
    type Error = Error;
    fn unit_variant(self) -> Result<()> {
        Ok(())
    }
    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(self.0)
    }
    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        track!(de::Deserializer::deserialize_map(self.0, visitor))
    }
}

fn status_code_to_str(code: u16) -> Result<&'static str> {
    Ok(match code {
        100 => "Continue",
        101 => "SwitchingProtocols",
        102 => "Processing",

        200 => "Ok",
        201 => "Created",
        202 => "Accepted",
        203 => "NonAuthoritativeInformation",
        204 => "NoContent",
        205 => "ResetContent",
        206 => "PartialContent",
        207 => "MultiStatus",
        208 => "AlreadyReported",
        226 => "ImUsed",

        300 => "MultipleChoices",
        301 => "MovedPermanently",
        302 => "Found",
        303 => "SeeOther",
        304 => "NotModified",
        305 => "UseProxy",
        307 => "TemporaryRedirect",
        308 => "PermanentRedirect",

        400 => "BadRequest",
        401 => "Unauthorized",
        402 => "PaymentRequired",
        403 => "Forbidden",
        404 => "NotFound",
        405 => "MethodNotAllowed",
        406 => "NotAcceptable",
        407 => "ProxyAuthenticationRequired",
        408 => "RequestTimeout",
        409 => "Conflict",
        410 => "Gone",
        411 => "LengthRequired",
        412 => "PreconditionFailed",
        413 => "PayloadTooLarge",
        414 => "UriTooLong",
        415 => "UnsupportedMediaType",
        416 => "RangeNotSatisfiable",
        417 => "ExceptionFailed",
        418 => "ImATeapot",
        421 => "MisdirectedRequest",
        422 => "UnprocessableEntity",
        423 => "Locked",
        424 => "FailedDependency",
        426 => "UpgradeRequired",
        451 => "UnavailableForLegalReasons",

        500 => "InternalServerError",
        501 => "NotImplemented",
        502 => "BadGateway",
        503 => "ServiceUnavailable",
        504 => "GatewayTimeout",
        505 => "HttpVersionNotSupported",
        506 => "VariantAlsoNegotiates",
        507 => "InsufficientStorage",
        508 => "LoopDetected",
        509 => "BandwidthLimitExceeded",
        510 => "NotExtended",
        _ => track_panic!(ErrorKind::Invalid, "Unknown HTTP status code: {}", code),
    })
}
