use futures::BoxFuture;
use serde::{Serialize, Deserialize};

use {Error, Method};
use path_template::PathTemplate;

pub trait Procedure {
    type Request: RpcRequest;
    type Response: RpcResponse;
    fn method() -> Method;
    fn entry_point() -> PathTemplate;
}

pub trait HandleRequest: Clone + Send + 'static {
    type Procedure: Procedure;
    fn handle_request(self,
                      request: <Self::Procedure as Procedure>::Request)
                      -> BoxFuture<<Self::Procedure as Procedure>::Response, Error>;
}

// TODO:
pub type EntryPoint = PathTemplate;

// TODO: doc for format convention
//
// TODO: s/RpcRequest/Request/
pub trait RpcRequest: Serialize + for<'a> Deserialize<'a> {}

// TODO: doc for format convention
//
// TODO: impl From<SystemError>
pub trait RpcResponse: Serialize + for<'a> Deserialize<'a> {}
