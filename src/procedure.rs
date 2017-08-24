use std::io::Read;
use futures::{Future, BoxFuture};
use miasht::builtin::io::BodyReader;
use serde::{Serialize, Deserialize};

use Error;
use types::HttpMethod;

/// Procedure definition.
pub trait Procedure {
    /// The request type of this procedure.
    type Request: RpcRequest;

    /// The response type of this procedure.
    type Response: RpcResponse;

    /// The HTTP method which this procedure handles.
    fn method() -> HttpMethod;

    /// The entry point of this procedure.
    fn entry_point() -> EntryPoint;
}

/// This trait allows to handle RPC requests issued by clients.
pub trait HandleRpc<P>: Clone + Send + 'static
where
    P: Procedure,
{
    /// The `Future` which represents the result of an invocation of the `handle_rpc` method.
    type Future: Future<Item = <P as Procedure>::Response, Error = NeverFail> + Send + 'static;

    /// Handles an RPC request issued by a client.
    fn handle_rpc(self, request: <P as Procedure>::Request) -> Self::Future;
}

/// A marker type used to indicate that a future never fails.
pub struct NeverFail {
    _cannot_instantiate: (),
}

/// RPC Request.
///
/// Implementations of this trait have to follow conventions as follows.
///
/// ```no_run
/// extern crate htrpc;
/// extern crate serde;
/// #[macro_use]
/// extern crate serde_derive;
///
/// use htrpc::RpcRequest;
///
/// #[derive(Serialize, Deserialize)]
/// struct FooRequest {
///     // This field will be processed by `UrlPathSerializer` and `UrlPathDeserializer`.
///     // (optional)
///     //
///     // Note that the arity of this tuple must be equal to
///     // the count of variables in the entry point of the RPC.
///     path: (Arg0, Arg1),
///
///     // This field will be processed by `UrlQuerySerializer` and `UrlQueryDeserializer`.
///     // (optional)
///     query: Query,
///
///     // This field will be processed by `HttpHeaderSerializer` and `HttpHeaderDeserializer`.
///     // (optional)
///     //
///     // Note that the following header fields are automatically inserted:
///     // - Content-Length
///     header: Header,
///
///     // This field will be processed by `HttpBodySerializer` and `HttpBodyDeserializer`.
///     // (optional)
///     //
///     // If you want to specify a more structured object as the body of RPC request,
///     // please consider to use the `#[serde(with = ...)]` attribute.
///     body: Vec<u8>,
/// }
/// impl RpcRequest for FooRequest {}
///
/// #[derive(Serialize, Deserialize)]
/// struct Arg0(String);
///
/// #[derive(Serialize, Deserialize)]
/// struct Arg1(usize);
///
/// #[derive(Serialize, Deserialize)]
/// struct Query {
///   key1: String,
///   key2: Option<u8>,
/// }
///
/// #[derive(Serialize, Deserialize)]
/// struct Header {
///   #[serde(rename = "X-Foo")]
///   foo: String
/// }
/// # fn main() {}
/// ```
pub trait RpcRequest: Serialize + for<'a> Deserialize<'a> + Send + 'static {
    /// Returns the body of this HTTP response.
    fn body(&mut self) -> Vec<u8>;

    /// Reads the body of this HTTP response.
    fn read_body<R: Read>(self, body: BodyReader<R>) -> BoxFuture<(BodyReader<R>, Self), Error>;
}

/// RPC Response.
///
/// Implementations of this trait have to follow conventions as follows.
///
/// ```no_run
/// extern crate htrpc;
/// extern crate serde;
/// #[macro_use]
/// extern crate serde_derive;
///
/// use htrpc::RpcResponse;
///
/// #[derive(Serialize, Deserialize)]
/// enum FooResponse {
///     Ok { header: Header, body: Vec<u8> },
///     NotFound { header: Header },
///     MethodNotAllowed,
///     InternalServerError { body: String },
/// }
/// impl RpcResponse for FooResponse {}
///
/// #[derive(Serialize, Deserialize)]
/// struct Header {
///   #[serde(rename = "X-Foo")]
///   foo: String
/// }
/// # fn main() {}
/// ```
pub trait RpcResponse: Serialize + for<'a> Deserialize<'a> {
    /// Returns the body of this HTTP response.
    fn body(&mut self) -> Box<AsRef<[u8]> + Send + 'static>;

    /// Sets the body of this HTTP response.
    fn set_body(&mut self, body: Vec<u8>);
}

/// The entry point definition of a procedure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntryPoint {
    segments: &'static [PathSegment],
}
impl EntryPoint {
    /// Makes a new `EntryPoint` instance.
    ///
    /// Usually it is convenient to use `htrpc_entry_point!` macro to construct this.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate htrpc;
    /// use htrpc::types::{EntryPoint, PathSegment};
    ///
    /// # fn main() {
    /// static SEGMENTS: &[PathSegment] =
    ///     &[PathSegment::Val("foo"), PathSegment::Var, PathSegment::Val("baz")];
    /// let p0 = EntryPoint::new(SEGMENTS);
    /// let p1 = htrpc_entry_point!["foo", _, "baz"];
    /// assert_eq!(p0, p1);
    /// # }
    /// ```
    pub fn new(segments: &'static [PathSegment]) -> Self {
        EntryPoint { segments }
    }

    /// Returns the segments in this entry point.
    pub fn segments(&self) -> &'static [PathSegment] {
        self.segments
    }

    /// Counts variables in this entry point.
    pub fn var_count(&self) -> usize {
        self.segments
            .iter()
            .filter(|s| s == &&PathSegment::Var)
            .count()
    }
}

/// Path segment which is used for constructing `EntryPoint`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PathSegment {
    /// Value (i.e., constant) segment.
    Val(&'static str),

    /// Variable (i.e., wildcard) segment.
    Var,
}
impl PathSegment {
    /// Converts to `Option`.
    pub fn as_option(&self) -> Option<&'static str> {
        if let PathSegment::Val(s) = *self {
            Some(s)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        use self::PathSegment::*;
        static SEGMENTS: &[PathSegment] = &[Val("foo"), Var, Val("baz")];
        let path0 = EntryPoint::new(SEGMENTS);
        let path1 = htrpc_entry_point!["foo", _, "baz"];
        assert_eq!(path0, path1);
    }
}
