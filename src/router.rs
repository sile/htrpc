use std::collections::HashMap;
use std::sync::Arc;
use fibers::net::TcpStream;
use futures::BoxFuture;
use miasht::server::{Request, Response};
use url::Url;

use {Result, Error, ErrorKind};
use procedure::EntryPoint;
use types::HttpMethod;

type HandleHttpRequestResult = BoxFuture<(Response<TcpStream>, Vec<u8>), Error>;
type HandleHttpRequest =
    Box<Fn(Url, Request<TcpStream>, Vec<u8>) -> HandleHttpRequestResult + Send + 'static>;

#[derive(Debug)]
pub enum RouteError {
    NotFound,
    MethodNotAllowed,
}

#[derive(Clone)]
pub struct Router {
    trie: Arc<Trie>,
}
unsafe impl Send for Router {}
impl Router {
    pub fn route(&self,
                 url: &Url,
                 request: &Request<TcpStream>)
                 -> ::std::result::Result<&HandleHttpRequest, RouteError> {
        let mut trie = self.trie.root();
        for segment in url.path_segments().expect("Never fails") {
            if let Some(child) = trie.get_child(segment) {
                trie = child;
            } else {
                return Err(RouteError::NotFound);
            }
        }
        trie.get_value(request.method())
            .ok_or(RouteError::MethodNotAllowed)
    }
}

pub struct RouterBuilder {
    trie: Trie,
}
impl RouterBuilder {
    pub fn new() -> Self {
        RouterBuilder { trie: Trie::new() }
    }
    pub fn finish(self) -> Router {
        Router { trie: Arc::new(self.trie) }
    }
    pub fn register_handler<H: Send + 'static>(&mut self,
                                               method: HttpMethod,
                                               entry_point: EntryPoint,
                                               handler: H)
                                               -> Result<()>
        where H: Fn(Url, Request<TcpStream>, Vec<u8>) -> HandleHttpRequestResult
    {
        track_try!(self.trie.insert(method, &entry_point, Box::new(handler)));
        Ok(())
    }
}

struct Trie {
    root: TrieNode,
}
impl Trie {
    pub fn new() -> Self {
        Trie { root: TrieNode::new() }
    }
    pub fn insert(&mut self,
                  method: HttpMethod,
                  entry_point: &EntryPoint,
                  handler: HandleHttpRequest)
                  -> Result<()> {
        let mut node = &mut self.root;
        for segment in entry_point.segments() {
            use types::PathSegment::*;
            let key = match *segment {
                Val(s) => Some(s),
                Var => None,
            };
            let prev = node;
            node = prev.children.entry(key).or_insert_with(|| TrieNode::new());
        }
        track_assert!(!node.leafs.contains_key(&method),
                      ErrorKind::Invalid,
                      "Conflicted: entry_point={:?}, method={:?}",
                      entry_point,
                      method);
        node.leafs.insert(method, handler);
        Ok(())
    }
    pub fn root(&self) -> &TrieNode {
        &self.root
    }
}

struct TrieNode {
    children: HashMap<Option<&'static str>, TrieNode>,
    leafs: HashMap<HttpMethod, HandleHttpRequest>,
}
impl TrieNode {
    pub fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            leafs: HashMap::new(),
        }
    }
    pub fn get_child<'a>(&'a self, segment: &str) -> Option<&'a Self> {
        let segment: &'static str = unsafe { &*(segment as *const _) as _ };
        self.children
            .get(&Some(segment))
            .or_else(|| self.children.get(&None))
    }
    pub fn get_value(&self, method: HttpMethod) -> Option<&HandleHttpRequest> {
        self.leafs.get(&method)
    }
}
