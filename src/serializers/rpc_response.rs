use fibers::net::TcpStream;
use miasht::builtin::headers;
use miasht::server::{Connection, ResponseBuilder, Response};
use miasht::status::RawStatus;
use serde::{ser, Serialize};
use serde::ser::Impossible;
use serdeconv;

use {Result, Error, ErrorKind};
use serializers::{HttpBodySerializer, HttpHeaderSerializer};
use types::HttpStatus;

/// `Serializer` implementation for RPC response.
#[derive(Debug)]
pub struct RpcResponseSerializer {
    connection: Option<Connection<TcpStream>>,
    response: Option<ResponseBuilder<TcpStream>>,
    body: Vec<u8>,
}
impl RpcResponseSerializer {
    /// Serializes the RPC response.
    pub fn serialize<T>(rpc_response: T,
                        connection: Connection<TcpStream>)
                        -> Result<(Response<TcpStream>, Vec<u8>)>
        where T: Serialize
    {
        let mut serializer = RpcResponseSerializer::new(connection);
        track_try!(rpc_response.serialize(&mut serializer));
        track!(serializer.finish())
    }

    /// Makes a new `RpcResponseSerializer` instance.
    pub fn new(connection: Connection<TcpStream>) -> Self {
        RpcResponseSerializer {
            connection: Some(connection),
            response: None,
            body: Vec::new(),
        }
    }

    /// Finishes the serialization and returns the resulting HTTP response and body.
    pub fn finish(self) -> Result<(Response<TcpStream>, Vec<u8>)> {
        track_assert!(self.response.is_some(), ErrorKind::Invalid);
        let mut response = self.response.expect("Never fail");
        response.add_header(&headers::ContentLength(self.body.len() as u64));
        Ok((response.finish(), self.body))
    }
}
impl<'a> ser::Serializer for &'a mut RpcResponseSerializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_i8(self, _v: i8) -> Result<Self::Ok> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_i16(self, _v: i16) -> Result<Self::Ok> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_i32(self, _v: i32) -> Result<Self::Ok> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_i64(self, _v: i64) -> Result<Self::Ok> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_u8(self, _v: u8) -> Result<Self::Ok> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_u16(self, _v: u16) -> Result<Self::Ok> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_u32(self, _v: u32) -> Result<Self::Ok> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_u64(self, _v: u64) -> Result<Self::Ok> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_f32(self, _v: f32) -> Result<Self::Ok> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_f64(self, _v: f64) -> Result<Self::Ok> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_char(self, _v: char) -> Result<Self::Ok> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_str(self, _v: &str) -> Result<Self::Ok> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_none(self) -> Result<Self::Ok> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok>
        where T: ?Sized + Serialize
    {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_unit(self) -> Result<Self::Ok> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_unit_variant(mut self,
                              _name: &'static str,
                              _variant_index: u32,
                              variant: &'static str)
                              -> Result<Self::Ok> {
        track_assert!(self.connection.is_some(), ErrorKind::Invalid);
        let status = track_try!(status_from_str(variant));
        let response = self.connection.take().unwrap().build_response(status);
        self.response = Some(response);
        Ok(())
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
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
    }
    fn serialize_struct_variant(self,
                                _name: &'static str,
                                _variant_index: u32,
                                variant: &'static str,
                                _len: usize)
                                -> Result<Self::SerializeStructVariant> {
        track_assert!(self.connection.is_some(), ErrorKind::Invalid);
        let status = track_try!(status_from_str(variant));
        let response = self.connection.take().unwrap().build_response(status);
        self.response = Some(response);
        Ok(self)
    }
}
impl<'a> ser::SerializeStruct for &'a mut RpcResponseSerializer {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        match key {
            "status" => {
                track_assert!(self.connection.is_some(), ErrorKind::Invalid);
                let status = track_try!(serdeconv::to_json_string(value));
                let status = track_try!(status.parse());
                let status = track_try!(RawStatus::new(status, "")
                                            .normalize()
                                            .ok_or(ErrorKind::Invalid),
                                        "Unknown HTTP status: {}",
                                        status);
                let response = self.connection.take().unwrap().build_response(status);
                self.response = Some(response);
                Ok(())
            }
            "header" => {
                track_assert!(self.connection.is_none(), ErrorKind::Invalid);
                let mut response = self.response.as_mut().unwrap();
                let mut serializer = HttpHeaderSerializer::new(response.headers_mut());
                track_try!(value.serialize(&mut serializer));
                Ok(())
            }
            "body" => {
                track_assert!(self.connection.is_none(), ErrorKind::Invalid);
                let body = track_try!(value.serialize(HttpBodySerializer));
                self.body = body;
                Ok(())
            }
            _ => track_panic!(ErrorKind::Invalid, "Unknown field: {:?}", key),
        }
    }
    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}
impl<'a> ser::SerializeStructVariant for &'a mut RpcResponseSerializer {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        match key {
            "header" => {
                let mut response = self.response.as_mut().unwrap();
                let mut serializer = HttpHeaderSerializer::new(response.headers_mut());
                track_try!(value.serialize(&mut serializer));
                Ok(())
            }
            "body" => {
                let body = track_try!(value.serialize(HttpBodySerializer));
                self.body = body;
                Ok(())
            }
            _ => track_panic!(ErrorKind::Invalid, "Unknown field: {:?}", key),
        }
    }
    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

fn status_from_str(s: &str) -> Result<HttpStatus> {
    Ok(match s {
           "Continue" => HttpStatus::Continue,
           "SwitchingProtocols" => HttpStatus::SwitchingProtocols,
           "Processing" => HttpStatus::Processing,
           "Ok" => HttpStatus::Ok,
           "Created" => HttpStatus::Created,
           "Accepted" => HttpStatus::Accepted,
           "NonAuthoritativeInformation" => HttpStatus::NonAuthoritativeInformation,
           "NoContent" => HttpStatus::NoContent,
           "ResetContent" => HttpStatus::ResetContent,
           "PartialContent" => HttpStatus::PartialContent,
           "MultiStatus" => HttpStatus::MultiStatus,
           "AlreadyReported" => HttpStatus::AlreadyReported,
           "ImUsed" => HttpStatus::ImUsed,
           "MultipleChoices" => HttpStatus::MultipleChoices,
           "MovedPermanently" => HttpStatus::MovedPermanently,
           "Found" => HttpStatus::Found,
           "SeeOther" => HttpStatus::SeeOther,
           "NotModified" => HttpStatus::NotModified,
           "UseProxy" => HttpStatus::UseProxy,
           "TemporaryRedirect" => HttpStatus::TemporaryRedirect,
           "PermanentRedirect" => HttpStatus::PermanentRedirect,
           "BadRequest" => HttpStatus::BadRequest,
           "Unauthorized" => HttpStatus::Unauthorized,
           "PaymentRequired" => HttpStatus::PaymentRequired,
           "Forbidden" => HttpStatus::Forbidden,
           "NotFound" => HttpStatus::NotFound,
           "MethodNotAllowed" => HttpStatus::MethodNotAllowed,
           "NotAcceptable" => HttpStatus::NotAcceptable,
           "ProxyAuthenticationRequired" => HttpStatus::ProxyAuthenticationRequired,
           "RequestTimeout" => HttpStatus::RequestTimeout,
           "Conflict" => HttpStatus::Conflict,
           "Gone" => HttpStatus::Gone,
           "LengthRequired" => HttpStatus::LengthRequired,
           "PreconditionFailed" => HttpStatus::PreconditionFailed,
           "PayloadTooLarge" => HttpStatus::PayloadTooLarge,
           "UriTooLong" => HttpStatus::UriTooLong,
           "UnsupportedMediaType" => HttpStatus::UnsupportedMediaType,
           "RangeNotSatisfiable" => HttpStatus::RangeNotSatisfiable,
           "ExceptionFailed" => HttpStatus::ExceptionFailed,
           "ImATeapot" => HttpStatus::ImATeapot,
           "MisdirectedRequest" => HttpStatus::MisdirectedRequest,
           "UnprocessableEntity" => HttpStatus::UnprocessableEntity,
           "Locked" => HttpStatus::Locked,
           "FailedDependency" => HttpStatus::FailedDependency,
           "UpgradeRequired" => HttpStatus::UpgradeRequired,
           "UnavailableForLegalReasons" => HttpStatus::UnavailableForLegalReasons,
           "InternalServerError" => HttpStatus::InternalServerError,
           "NotImplemented" => HttpStatus::NotImplemented,
           "BadGateway" => HttpStatus::BadGateway,
           "ServiceUnavailable" => HttpStatus::ServiceUnavailable,
           "GatewayTimeout" => HttpStatus::GatewayTimeout,
           "HttpVersionNotSupported" => HttpStatus::HttpVersionNotSupported,
           "VariantAlsoNegotiates" => HttpStatus::VariantAlsoNegotiates,
           "InsufficientStorage" => HttpStatus::InsufficientStorage,
           "LoopDetected" => HttpStatus::LoopDetected,
           "BandwidthLimitExceeded" => HttpStatus::BandwidthLimitExceeded,
           "NotExtended" => HttpStatus::NotExtended,
           _ => track_panic!(ErrorKind::Invalid, "Unknown HTTP status: {:?}", s),
       })
}
