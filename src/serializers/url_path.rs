use serde::{ser, Serialize};
use serde::ser::Impossible;
use url::{Url, PathSegmentsMut};

use {Result, Error, ErrorKind, EntryPoint};

/// `Serializer` implementation for URL path.
pub struct UrlPathSerializer<'a> {
    segments: PathSegmentsMut<'a>,
    entry_point: &'a EntryPoint,
    index: usize,
    is_started: bool,
}
impl<'a> UrlPathSerializer<'a> {
    /// Makes a new `UrlPathSerializer` instance.
    pub fn new(entry_point: &'a EntryPoint, url: &'a mut Url) -> Result<Self> {
        let segments = track_try!(url.path_segments_mut().map_err(|_| ErrorKind::Invalid));
        Ok(UrlPathSerializer {
               segments,
               entry_point,
               index: 0,
               is_started: false,
           })
    }

    fn bind_next_var(&mut self, value: &str) -> Result<()> {
        track_assert!(self.is_started, ErrorKind::Invalid);
        track_assert!(!self.append_until_next_var(), ErrorKind::Invalid);
        self.segments.push(value);
        self.index += 1;
        Ok(())
    }
    fn finish(&mut self) -> Result<()> {
        track_assert!(self.is_started, ErrorKind::Invalid);
        track_assert!(self.append_until_next_var(), ErrorKind::Invalid);
        Ok(())
    }
    fn append_until_next_var(&mut self) -> bool {
        while self.index < self.entry_point.len() {
            if let Some(s) = self.entry_point.get_val(self.index) {
                self.segments.push(s);
                self.index += 1;
            } else {
                return false;
            }
        }
        true
    }
    fn var_count(&self) -> usize {
        self.entry_point.var_count()
    }
}
impl<'a, 'b> ser::Serializer for &'a mut UrlPathSerializer<'b> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        let s = if v { "ture" } else { "false" };
        track_try!(self.bind_next_var(s));
        Ok(())
    }
    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        track!(self.serialize_i64(v as i64))
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        track!(self.serialize_i64(v as i64))
    }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        track!(self.serialize_i64(v as i64))
    }
    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        track_try!(self.bind_next_var(&v.to_string()));
        Ok(())
    }
    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        track!(self.serialize_u64(v as u64))
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        track!(self.serialize_u64(v as u64))
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        track!(self.serialize_u64(v as u64))
    }
    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        track_try!(self.bind_next_var(&v.to_string()));
        Ok(())
    }
    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        track!(self.serialize_f64(v as f64))
    }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        track_try!(self.bind_next_var(&v.to_string()));
        Ok(())
    }
    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        track!(self.serialize_str(&v.to_string()))
    }
    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        track_try!(self.bind_next_var(v));
        Ok(())
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
        track_assert!(!self.is_started, ErrorKind::Invalid);
        track_assert_eq!(self.var_count(), 0, ErrorKind::Invalid);
        self.is_started = true;
        track_try!(self.finish());
        Ok(())
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        track_assert!(!self.is_started, ErrorKind::Invalid);
        track_assert_eq!(self.var_count(), 0, ErrorKind::Invalid);
        self.is_started = true;
        track_try!(self.finish());
        Ok(())
    }
    fn serialize_unit_variant(self,
                              _name: &'static str,
                              _variant_index: u32,
                              variant: &'static str)
                              -> Result<Self::Ok> {
        track!(self.serialize_str(variant))
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
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        track_assert!(!self.is_started, ErrorKind::Invalid);
        track_assert_eq!(self.var_count(), len, ErrorKind::Invalid);
        self.is_started = true;
        Ok(self)
    }
    fn serialize_tuple_struct(self,
                              _name: &'static str,
                              len: usize)
                              -> Result<Self::SerializeTupleStruct> {
        track_assert!(!self.is_started, ErrorKind::Invalid);
        track_assert_eq!(self.var_count(), len, ErrorKind::Invalid);
        self.is_started = true;
        Ok(self)
    }
    fn serialize_tuple_variant(self,
                               _name: &'static str,
                               _variant_index: u32,
                               _variant: &'static str,
                               len: usize)
                               -> Result<Self::SerializeTupleVariant> {
        track_assert!(!self.is_started, ErrorKind::Invalid);
        track_assert_eq!(self.var_count(), len, ErrorKind::Invalid);
        self.is_started = true;
        Ok(self)
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
                                _variant: &'static str,
                                _len: usize)
                                -> Result<Self::SerializeStructVariant> {
        track_panic!(ErrorKind::Invalid);
    }
}
impl<'a, 'b> ser::SerializeTuple for &'a mut UrlPathSerializer<'b> {
    type Ok = ();
    type Error = Error;
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        track_try!(value.serialize(&mut **self));
        Ok(())
    }
    fn end(self) -> Result<Self::Ok> {
        track_try!(self.finish());
        Ok(())
    }
}
impl<'a, 'b> ser::SerializeTupleStruct for &'a mut UrlPathSerializer<'b> {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        track_try!(value.serialize(&mut **self));
        Ok(())
    }
    fn end(self) -> Result<Self::Ok> {
        track_try!(self.finish());
        Ok(())
    }
}
impl<'a, 'b> ser::SerializeTupleVariant for &'a mut UrlPathSerializer<'b> {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        track_try!(value.serialize(&mut **self));
        Ok(())
    }
    fn end(self) -> Result<Self::Ok> {
        track_try!(self.finish());
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use url::Url;
    use super::*;

    #[test]
    fn it_works() {
        let entry_point = htrpc_entry_point!["foo", _, "baz", _];

        #[derive(Serialize)]
        struct Args(&'static str, usize);

        let mut url = Url::parse("http://localhost/").unwrap();
        {
            let mut serializer = track_try_unwrap!(UrlPathSerializer::new(&entry_point, &mut url));
            track_try_unwrap!(Args("hello world", 3).serialize(&mut serializer));
        }
        assert_eq!(url.as_str(), "http://localhost/foo/hello%20world/baz/3");
    }
}
