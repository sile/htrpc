use futures::Future;
use serde::{Serialize, Deserialize};

use {Error, Method};

pub trait Procedure {
    type Request: RpcRequest;
    type Response: RpcResponse;
    fn method() -> Method;
    fn entry_point() -> EntryPoint;
}

pub trait HandleRequest: Clone + Send + 'static {
    type Procedure: Procedure;
    type Future: Future<Item = <Self::Procedure as Procedure>::Response, Error = Error> + Send;
    fn handle_request(self, request: <Self::Procedure as Procedure>::Request) -> Self::Future;
}

// TODO: doc for format convention
//
// TODO: s/RpcRequest/Request/
pub trait RpcRequest: Serialize + for<'a> Deserialize<'a> {}

// TODO: doc for format convention
//
// TODO: impl From<SystemError>
pub trait RpcResponse: Serialize + for<'a> Deserialize<'a> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntryPoint {
    segments: &'static [PathSegment],
}
impl EntryPoint {
    pub fn new(segments: &'static [PathSegment]) -> Self {
        EntryPoint { segments }
    }
    pub fn var_count(&self) -> usize {
        self.segments
            .iter()
            .filter(|s| s == &&PathSegment::Var)
            .count()
    }
    pub fn len(&self) -> usize {
        self.segments.len()
    }
    pub fn is_var_remaning(&self, i: usize) -> bool {
        self.segments[i..]
            .iter()
            .find(|s| **s == PathSegment::Var)
            .is_some()
    }
    pub fn get_val(&self, i: usize) -> Option<&'static str> {
        if let PathSegment::Val(s) = self.segments[i] {
            Some(s)
        } else {
            None
        }
    }
    pub fn segments(&self) -> &'static [PathSegment] {
        self.segments
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PathSegment {
    Val(&'static str),
    Var,
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
