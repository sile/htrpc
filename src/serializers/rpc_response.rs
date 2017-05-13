use fibers::net::TcpStream;
use miasht::builtin::headers;
use miasht::server::{Connection, ResponseBuilder, Response};
use serde::{ser, Serialize};
use serde::ser::Impossible;

use {Result, Error, ErrorKind};
use serializers::{HttpBodySerializer, HttpHeaderSerializer};
use types::Status;

/// `Serializer` implementation for RPC response.
#[derive(Debug)]
pub struct RpcResponseSerializer {
    connection: Option<Connection<TcpStream>>,
    response: Option<ResponseBuilder<TcpStream>>,
    body: Vec<u8>,
}
impl RpcResponseSerializer {
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
        // TODO: Support keep-alive
        response.add_header(&headers::Connection::Close);
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
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
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
        track_panic!(ErrorKind::Invalid);
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

fn status_from_str(s: &str) -> Result<Status> {
    Ok(match s {
           "Continue" => Status::Continue,
           "SwitchingProtocols" => Status::SwitchingProtocols,
           "Processing" => Status::Processing,
           "Ok" => Status::Ok,
           "Created" => Status::Created,
           "Accepted" => Status::Accepted,
           "NonAuthoritativeInformation" => Status::NonAuthoritativeInformation,
           "NoContent" => Status::NoContent,
           "ResetContent" => Status::ResetContent,
           "PartialContent" => Status::PartialContent,
           "MultiStatus" => Status::MultiStatus,
           "AlreadyReported" => Status::AlreadyReported,
           "ImUsed" => Status::ImUsed,
           "MultipleChoices" => Status::MultipleChoices,
           "MovedPermanently" => Status::MovedPermanently,
           "Found" => Status::Found,
           "SeeOther" => Status::SeeOther,
           "NotModified" => Status::NotModified,
           "UseProxy" => Status::UseProxy,
           "TemporaryRedirect" => Status::TemporaryRedirect,
           "PermanentRedirect" => Status::PermanentRedirect,
           "BadRequest" => Status::BadRequest,
           "Unauthorized" => Status::Unauthorized,
           "PaymentRequired" => Status::PaymentRequired,
           "Forbidden" => Status::Forbidden,
           "NotFound" => Status::NotFound,
           "MethodNotAllowed" => Status::MethodNotAllowed,
           "NotAcceptable" => Status::NotAcceptable,
           "ProxyAuthenticationRequired" => Status::ProxyAuthenticationRequired,
           "RequestTimeout" => Status::RequestTimeout,
           "Conflict" => Status::Conflict,
           "Gone" => Status::Gone,
           "LengthRequired" => Status::LengthRequired,
           "PreconditionFailed" => Status::PreconditionFailed,
           "PayloadTooLarge" => Status::PayloadTooLarge,
           "UriTooLong" => Status::UriTooLong,
           "UnsupportedMediaType" => Status::UnsupportedMediaType,
           "RangeNotSatisfiable" => Status::RangeNotSatisfiable,
           "ExceptionFailed" => Status::ExceptionFailed,
           "ImATeapot" => Status::ImATeapot,
           "MisdirectedRequest" => Status::MisdirectedRequest,
           "UnprocessableEntity" => Status::UnprocessableEntity,
           "Locked" => Status::Locked,
           "FailedDependency" => Status::FailedDependency,
           "UpgradeRequired" => Status::UpgradeRequired,
           "UnavailableForLegalReasons" => Status::UnavailableForLegalReasons,
           "InternalServerError" => Status::InternalServerError,
           "NotImplemented" => Status::NotImplemented,
           "BadGateway" => Status::BadGateway,
           "ServiceUnavailable" => Status::ServiceUnavailable,
           "GatewayTimeout" => Status::GatewayTimeout,
           "HttpVersionNotSupported" => Status::HttpVersionNotSupported,
           "VariantAlsoNegotiates" => Status::VariantAlsoNegotiates,
           "InsufficientStorage" => Status::InsufficientStorage,
           "LoopDetected" => Status::LoopDetected,
           "BandwidthLimitExceeded" => Status::BandwidthLimitExceeded,
           "NotExtended" => Status::NotExtended,
           _ => track_panic!(ErrorKind::Invalid, "Unknown HTTP status: {:?}", s),
       })
}
