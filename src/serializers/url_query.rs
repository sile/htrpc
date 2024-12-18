use serde::ser::Impossible;
use serde::{ser, Serialize};
use std::borrow::Cow;
use url::form_urlencoded::Serializer;
use url::UrlQuery;

use {Error, ErrorKind, Result};

/// `Serializer` implementation for URL query string.
pub struct UrlQuerySerializer<'a> {
    is_first: bool,
    key: Option<Cow<'static, str>>,
    query: Serializer<'a, UrlQuery<'a>>,
}
impl<'a> UrlQuerySerializer<'a> {
    /// Makes a new `UrlQuerySerializer` instance.
    pub fn new(query: Serializer<'a, UrlQuery<'a>>) -> Self {
        UrlQuerySerializer {
            is_first: true,
            key: None,
            query,
        }
    }

    fn append(&mut self, key_or_val: Cow<str>) {
        if let Some(key) = self.key.take() {
            let val = &key_or_val;
            self.query.append_pair(&key, val);
        } else {
            let key = key_or_val.into_owned();
            self.key = Some(Cow::Owned(key));
        }
    }
}
impl<'a, 'b> ser::Serializer for &'a mut UrlQuerySerializer<'b> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        let s = if v { "ture" } else { "false" };
        self.append(Cow::Borrowed(s));
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
        self.append(Cow::Owned(v.to_string()));
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
        self.append(Cow::Owned(v.to_string()));
        Ok(())
    }
    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        track!(self.serialize_f64(v as f64))
    }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        self.append(Cow::Owned(v.to_string()));
        Ok(())
    }
    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        track!(self.serialize_str(&v.to_string()))
    }
    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        self.append(Cow::Borrowed(v));
        Ok(())
    }
    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_none(self) -> Result<Self::Ok> {
        track_assert!(self.key.take().is_some(), ErrorKind::Invalid);
        Ok(())
    }
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        track!(value.serialize(self))
    }
    fn serialize_unit(self) -> Result<Self::Ok> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        track!(self.serialize_str(variant))
    }
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        track!(value.serialize(self))
    }
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        track!(value.serialize(self))
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        track_panic!(ErrorKind::Invalid);
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        track_assert!(self.is_first, ErrorKind::Invalid);
        self.is_first = false;
        Ok(self)
    }
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        track_assert!(self.is_first, ErrorKind::Invalid);
        self.is_first = false;
        Ok(self)
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        track_assert!(self.is_first, ErrorKind::Invalid);
        self.is_first = false;
        Ok(self)
    }
}
impl<'a, 'b> ser::SerializeMap for &'a mut UrlQuerySerializer<'b> {
    type Ok = ();
    type Error = Error;
    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        track!(key.serialize(&mut **self))?;
        Ok(())
    }
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        track!(value.serialize(&mut **self))?;
        Ok(())
    }
    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}
impl<'a, 'b> ser::SerializeStruct for &'a mut UrlQuerySerializer<'b> {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.key = Some(Cow::Borrowed(key));
        track!(value.serialize(&mut **self))?;
        Ok(())
    }
    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}
impl<'a, 'b> ser::SerializeStructVariant for &'a mut UrlQuerySerializer<'b> {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.key = Some(Cow::Borrowed(key));
        track!(value.serialize(&mut **self))?;
        Ok(())
    }
    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::BTreeMap;
    use url::Url;

    #[test]
    fn map_works() {
        let mut url = Url::parse("http://localhost/").unwrap();
        {
            let mut serializer = UrlQuerySerializer::new(url.query_pairs_mut());
            let params: BTreeMap<_, _> =
                [("foo", "3"), ("bar", "baz qux")].iter().cloned().collect();
            params.serialize(&mut serializer).unwrap();
        }
        assert_eq!(url.as_str(), "http://localhost/?bar=baz+qux&foo=3");
    }

    #[test]
    fn struct_works() {
        #[derive(Serialize)]
        struct Params {
            foo: Option<usize>,
            bar: &'static str,
        }

        let mut url = Url::parse("http://localhost/").unwrap();
        {
            let mut serializer = UrlQuerySerializer::new(url.query_pairs_mut());
            let params = Params {
                foo: None,
                bar: "baz qux",
            };
            params.serialize(&mut serializer).unwrap();
        }
        assert_eq!(url.as_str(), "http://localhost/?bar=baz+qux");

        let mut url = Url::parse("http://localhost/").unwrap();
        {
            let mut serializer = UrlQuerySerializer::new(url.query_pairs_mut());
            let params = Params {
                foo: Some(3),
                bar: "baz qux",
            };
            params.serialize(&mut serializer).unwrap();
        }
        assert_eq!(url.as_str(), "http://localhost/?foo=3&bar=baz+qux");
    }

    #[test]
    fn struct_variant_works() {
        #[allow(dead_code)]
        #[derive(Serialize)]
        enum Params {
            Abb,
            Bbb { foo: usize, bar: &'static str },
        }

        let mut url = Url::parse("http://localhost/").unwrap();
        {
            let mut serializer = UrlQuerySerializer::new(url.query_pairs_mut());
            let params = Params::Bbb {
                foo: 3,
                bar: "baz qux",
            };
            params.serialize(&mut serializer).unwrap();
        }
        assert_eq!(url.as_str(), "http://localhost/?foo=3&bar=baz+qux");
    }
}
