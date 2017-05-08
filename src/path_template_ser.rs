use serde::{ser, Serialize};
use url::percent_encoding;

use {Result, Error, ErrorKind};

#[derive(Debug)]
pub struct Unsupported {
    _cannot_instantiate: (),
}
impl ser::SerializeTuple for Unsupported {
    type Ok = ();
    type Error = Error;
    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        unreachable!();
    }
    fn end(self) -> Result<Self::Ok> {
        unreachable!();
    }
}
impl ser::SerializeTupleStruct for Unsupported {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        unreachable!();
    }
    fn end(self) -> Result<Self::Ok> {
        unreachable!();
    }
}
impl ser::SerializeTupleVariant for Unsupported {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        unreachable!();
    }
    fn end(self) -> Result<Self::Ok> {
        unreachable!();
    }
}
impl ser::SerializeStructVariant for Unsupported {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        unreachable!();
    }
    fn end(self) -> Result<Self::Ok> {
        unreachable!();
    }
}

#[derive(Debug)]
pub struct Serializer {
    value: String,
}
impl Serializer {
    pub fn new() -> Self {
        Serializer { value: String::new() }
    }
}
impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Unsupported;
    type SerializeTupleStruct = Unsupported;
    type SerializeTupleVariant = Unsupported;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Unsupported;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        self.value = v.to_string();
        Ok(())
    }
    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        self.serialize_i64(v as i64)
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        self.serialize_i64(v as i64)
    }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        self.serialize_i64(v as i64)
    }
    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        self.value = v.to_string();
        Ok(())
    }
    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        self.serialize_u64(v as u64)
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        self.serialize_u64(v as u64)
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        self.serialize_u64(v as u64)
    }
    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        self.value = v.to_string();
        Ok(())
    }
    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        self.serialize_f64(v as f64)
    }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        self.value = v.to_string();
        Ok(())
    }
    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        self.serialize_str(&v.to_string())
    }
    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        let encoded =
            percent_encoding::utf8_percent_encode(v, percent_encoding::PATH_SEGMENT_ENCODE_SET)
                .collect();
        self.value = encoded;
        Ok(())
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        let encoded = percent_encoding::percent_encode(v,
                                                       percent_encoding::PATH_SEGMENT_ENCODE_SET)
                .collect();
        self.value = encoded;

        Ok(())
    }
    fn serialize_none(self) -> Result<Self::Ok> {
        track_panic!(ErrorKind::Unsupported);
    }
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
        where T: ?Sized + Serialize
    {
        value.serialize(self)
    }
    fn serialize_unit(self) -> Result<Self::Ok> {
        track_panic!(ErrorKind::Unsupported);
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        track_panic!(ErrorKind::Unsupported);
    }
    fn serialize_unit_variant(self,
                              _name: &'static str,
                              _variant_index: u32,
                              variant: &'static str)
                              -> Result<Self::Ok> {
        self.serialize_str(variant)
    }
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
        where T: ?Sized + Serialize
    {
        value.serialize(self)
    }
    fn serialize_newtype_variant<T>(self,
                                    _name: &'static str,
                                    _variant_index: u32,
                                    _variant: &'static str,
                                    _value: &T)
                                    -> Result<Self::Ok>
        where T: ?Sized + Serialize
    {
        track_panic!(ErrorKind::Unsupported);
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        track_panic!(ErrorKind::Unsupported);
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        track_panic!(ErrorKind::Unsupported);
    }
    fn serialize_tuple_struct(self,
                              _name: &'static str,
                              _len: usize)
                              -> Result<Self::SerializeTupleStruct> {
        track_panic!(ErrorKind::Unsupported);
    }
    fn serialize_tuple_variant(self,
                               _name: &'static str,
                               _variant_index: u32,
                               _variant: &'static str,
                               _len: usize)
                               -> Result<Self::SerializeTupleVariant> {
        track_panic!(ErrorKind::Unsupported);
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        // TODO
        println!("[{}:{}] serialize_map", file!(), line!());
        Ok(self)
    }
    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }
    fn serialize_struct_variant(self,
                                _name: &'static str,
                                _variant_index: u32,
                                _variant: &'static str,
                                _len: usize)
                                -> Result<Self::SerializeStructVariant> {
        track_panic!(ErrorKind::Unsupported);
    }
}
impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        println!("[{}:{}] serialize_element", file!(), line!());
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<Self::Ok> {
        println!("[{}:{}] end", file!(), line!());
        Ok(())
    }
}
impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;
    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        println!("[{}:{}] serialize_key", file!(), line!());
        key.serialize(&mut **self)
    }
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        println!("[{}:{}] serialize_value", file!(), line!());
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<Self::Ok> {
        println!("[{}:{}] end", file!(), line!());
        Ok(())
    }
}
impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        println!("[{}:{}] serialize_field", file!(), line!());
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<Self::Ok> {
        println!("[{}:{}] end", file!(), line!());
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use url::Url;

    use path_template::{PathTemplate, PathSegment};

    #[test]
    fn it_works() {
        use path_template::PathSegment::*;
        static SEGMENTS: &[PathSegment] = &[Val("foo"), Var("bar"), Val("baz")];
        let path = PathTemplate::new(SEGMENTS);

        #[derive(Serialize)]
        struct Params {
            pub bar: usize,
        }

        let mut url = Url::parse("http://localhost/").unwrap();
        path.fill_path_segments(&mut url, Params { bar: 10 })
            .unwrap();
    }
}
