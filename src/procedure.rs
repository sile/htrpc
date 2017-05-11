use futures::BoxFuture;
use serde::{Serialize, Deserialize};

#[allow(unused_imports)]
use {Result, Method, Status};
use path_template::PathTemplate;

// TODO
#[derive(Debug)]
pub struct Unreachable {
    _cannot_instantiate: (),
}

pub trait Procedure {
    // TODO: s/input/request/
    type Input: RpcInput;
    type Output: RpcOutput;
    fn entry_point() -> EntryPoint;
}
pub trait HandleCall: Clone + Send + 'static {
    type Procedure: Procedure;
    fn handle_call(self,
                   input: <Self::Procedure as Procedure>::Input)
                   -> BoxFuture<<Self::Procedure as Procedure>::Output, Unreachable>;
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

// enum Response {
//     Ok{header, body},
//     NotFound{header},
//     Default{body},
// }
pub trait RpcOutput: Serialize + for<'a> Deserialize<'a> {}
