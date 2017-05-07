use futures::BoxFuture;
use serde::{Serialize, Deserialize};

use {Result, Method, Status};

pub type Unreachable = ();
pub type Request = ();

pub trait Procedure {
    type Input: RpcInput;
    type Output: RpcOutput;
    fn entry_point() -> EntryPoint;
    fn handle_call(self, input: Self::Input) -> BoxFuture<Self::Output, Unreachable>;
}

#[derive(Debug)]
pub struct EntryPoint {
    pub method: Method,
    pub path: &'static [Option<&'static str>],
}

#[derive(Debug)]
pub enum RpcBody<T> {
    Raw(Vec<u8>),
    Json(T),
    MsgPack(T),
}

pub trait RpcInput: Sized {
    type Path: Serialize + for<'a> Deserialize<'a>;
    type Query: Serialize + for<'a> Deserialize<'a>;
    type Header: Serialize + for<'a> Deserialize<'a>;
    type Body: Serialize + for<'a> Deserialize<'a>;

    fn compose(path: Self::Path,
               query: Self::Query,
               header: Self::Header,
               body: Self::Body)
               -> Result<Self>;
    fn decompose(self) -> Result<(Self::Path, Self::Query, Self::Header, RpcBody<Self::Body>)>;
}

pub trait RpcOutput: Sized {
    type Header: Serialize + for<'a> Deserialize<'a>;
    type Body: Serialize + for<'a> Deserialize<'a>;

    fn compose(status: Status, header: Self::Header, body: Self::Body) -> Result<Self>;
    fn decompose(self) -> Result<(Status, Self::Header, RpcBody<Self::Body>)>;
}
