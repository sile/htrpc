use fibers::net::TcpStream;
use miasht::builtin::headers::ContentLength;
use miasht::client::{Connection, RequestBuilder, Request};
use serde::{ser, Serialize};
use serde::ser::Impossible;
use url::{self, Url};

use {Result, Error, ErrorKind, Method};
use procedure::EntryPoint;
use serializers::{UrlPathSerializer, UrlQuerySerializer, HttpBodySerializer, HttpHeaderSerializer};

/// `Serializer` implementation for RPC request.
#[derive(Debug)]
pub struct RpcRequestSerializer {
    temp_url: Url,
    is_path_initialized: bool,
    method: Method,
    entry_point: EntryPoint,
    connection: Option<Connection<TcpStream>>,
    request: Option<RequestBuilder<TcpStream>>,
    body: Vec<u8>,
}
impl RpcRequestSerializer {
    /// Makes a new `RpcRequestSerializer` instance.
    pub fn new(connection: Connection<TcpStream>, method: Method, entry_point: EntryPoint) -> Self {
        RpcRequestSerializer {
            temp_url: Url::parse("http://localhost/").expect("Never fail"),
            is_path_initialized: false,
            method,
            entry_point,
            connection: Some(connection),
            request: None,
            body: Vec::new(),
        }
    }

    /// Finishes the serialization and returns the resulting HTTP request and body.
    pub fn finish(self) -> Result<(Request<TcpStream>, Vec<u8>)> {
        track_assert!(self.request.is_some(), ErrorKind::Invalid);
        let mut request = self.request.unwrap();
        request.add_header(&ContentLength(self.body.len() as u64));
        Ok((request.finish(), self.body))
    }
}
impl<'a> ser::Serializer for &'a mut RpcRequestSerializer {
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
    fn serialize_unit_variant(self,
                              _name: &'static str,
                              _variant_index: u32,
                              _variant: &'static str)
                              -> Result<Self::Ok> {
        track_panic!(ErrorKind::Invalid);
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
                                name: &'static str,
                                _variant_index: u32,
                                _variant: &'static str,
                                len: usize)
                                -> Result<Self::SerializeStructVariant> {
        track!(self.serialize_struct(name, len))
    }
}
impl<'a> ser::SerializeStruct for &'a mut RpcRequestSerializer {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        match key {
            "path" => {
                track_assert!(self.connection.is_some(), ErrorKind::Invalid);
                {
                    let mut serializer = track_try!(UrlPathSerializer::new(&self.entry_point,
                                                                           &mut self.temp_url));
                    track_try!(value.serialize(&mut serializer));
                }
                self.is_path_initialized = true;
                Ok(())
            }
            "query" => {
                track_assert!(self.connection.is_some(), ErrorKind::Invalid);
                if !self.is_path_initialized {
                    let mut serializer = track_try!(UrlPathSerializer::new(&self.entry_point,
                                                                           &mut self.temp_url));
                    track_try!(value.serialize(&mut serializer));
                    self.is_path_initialized = true;
                }
                {
                    let mut serializer = UrlQuerySerializer::new(self.temp_url.query_pairs_mut());
                    track_try!(value.serialize(&mut serializer));
                }
                let relative_url = &self.temp_url[url::Position::BeforePath..];
                let connection = self.connection.take().unwrap();
                self.request = Some(connection.build_request(self.method, relative_url));
                Ok(())
            }
            "header" => {
                let mut request = self.request.as_mut().unwrap();
                let mut serializer = HttpHeaderSerializer::new(request.headers_mut());
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
impl<'a> ser::SerializeStructVariant for &'a mut RpcRequestSerializer {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        match key {
            "header" => {
                let mut request = self.request.as_mut().unwrap();
                let mut serializer = HttpHeaderSerializer::new(request.headers_mut());
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
