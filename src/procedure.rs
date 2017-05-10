use futures::BoxFuture;
use serde::{Serialize, Deserialize};

use {Result, Method, Status};
use path_template::PathTemplate;

// TODO
pub type Unreachable = ();
pub type Request = ();

pub trait Procedure {
    // TODO: s/input/request/
    type Input: RpcInput;
    type Output: RpcOutput;
    fn entry_point() -> EntryPoint;
    fn handle_call(self, input: Self::Input) -> BoxFuture<Self::Output, Unreachable>;
}

#[derive(Debug)]
pub struct EntryPoint {
    pub method: Method,
    pub path: PathTemplate,
}

pub trait ContentType {
    fn mime() -> Option<&'static str>;
    fn serialize_body<T>(body: T) -> Result<Vec<u8>> where T: Serialize;
    fn deserialize_body<T>(bytes: Vec<u8>) -> Result<T> where T: for<'a> Deserialize<'a>;
}

pub trait RpcInputBody: Serialize + for<'a> Deserialize<'a> {
    type ContentType: ContentType;
}

pub trait RpcInput: Sized {
    type Path: Serialize + for<'a> Deserialize<'a>;
    type Query: Serialize + for<'a> Deserialize<'a>;
    type Header: Serialize + for<'a> Deserialize<'a>;
    type Body: RpcInputBody;

    fn compose(path: Self::Path,
               query: Self::Query,
               header: Self::Header,
               body: Self::Body)
               -> Result<Self>;
    fn decompose(self) -> Result<(Self::Path, Self::Query, Self::Header, Self::Body)>;

    fn content_type() -> Option<&'static str> {
        <Self::Body as RpcInputBody>::ContentType::mime()
    }
    fn serialize_body<T>(body: T) -> Result<Vec<u8>>
        where T: Serialize
    {
        <Self::Body as RpcInputBody>::ContentType::serialize_body(body)
    }
    fn deserialize_body<T>(bytes: Vec<u8>) -> Result<T>
        where T: for<'a> Deserialize<'a>
    {
        <Self::Body as RpcInputBody>::ContentType::deserialize_body(bytes)
    }
}

pub trait RpcOutput: Sized {
    type Header: Serialize + for<'a> Deserialize<'a>;
    type Body: Serialize + for<'a> Deserialize<'a>;

    fn compose(status: Status, header: Self::Header, body: Self::Body) -> Result<Self>;
    fn decompose(self) -> Result<(Status, Self::Header, Self::Body)>;
}
