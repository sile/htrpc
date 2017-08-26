use futures::Future;
use serde::{Serialize, Deserialize};

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
pub trait RpcRequest: Serialize + for<'a> Deserialize<'a> + Send + 'static {
    /// Returns the body of this HTTP response.
    fn body(&mut self) -> Vec<u8>;

    /// Reads the body of this HTTP response.
    fn read_body(self, body: ::BodyReader) -> ::ReadBody<Self>;
}

/// RPC Response.
///
/// Implementations of this trait have to follow conventions as follows.
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
