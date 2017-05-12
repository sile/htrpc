use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use fibers::{self, Spawn, BoxSpawn};
use fibers::net::TcpStream;
use futures::{self, Future, BoxFuture, Poll, Async, Stream};
use futures::stream::StreamFuture;
use handy_async::future::Phase;
use miasht;
use miasht::builtin::io::IoExt;
use miasht::builtin::futures::FutureExt;
use miasht::server::{Request, Response, Connection};
use serde::Deserialize;
use url::Url;

use {Result, Error, Method, ErrorKind};
use deserializers::RequestDeserializer;
use procedure::{Procedure, HandleRequest, EntryPoint};
use serializers::ResponseSerializer;

pub struct RpcServerBuilder {
    bind_addr: SocketAddr,
    router: RoutingTreeBuilder,
}
impl RpcServerBuilder {
    pub fn new(bind_addr: SocketAddr) -> Self {
        RpcServerBuilder {
            bind_addr,
            router: RoutingTreeBuilder::new(),
        }
    }
    pub fn register<H>(&mut self, handler: H) -> Result<()>
        where H: HandleRequest
    {
        let handle_request = move |url, request, body| -> HandleRequestResult {
            // TODO: error handling
            let rpc_request =
                track_try_unwrap!(request_to_rpc_request::<H::Procedure>(&url, &request, body));
            let handler = handler.clone();
            handler
                .handle_request(rpc_request)
                .then(move |result| {
                          let rpc_response = result.expect("Unreachable");
                          let response =
                              track_try_unwrap!(rpc_response_to_response::<H::Procedure>(request.finish(),
                                                                                   rpc_response));
                          Ok(response)
                      })
                .boxed()
        };
        track_try!(self.router
                   .register(H::Procedure::method(), H::Procedure::entry_point(), handle_request));
        Ok(())
    }
    pub fn start<S>(self, spawner: S) -> RpcServer
        where S: Spawn + Send + 'static
    {
        RpcServer {
            spawner: spawner.boxed(),
            router: self.router.finish(),
            phase: Phase::A(fibers::net::TcpListener::bind(self.bind_addr)),
        }
    }
}

pub struct RpcServer {
    spawner: BoxSpawn,
    router: RoutingTree,
    phase:
        Phase<fibers::net::futures::TcpListenerBind, StreamFuture<fibers::net::streams::Incoming>>,
}
impl Future for RpcServer {
    type Item = ();
    type Error = Error;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            let next = match track_try!(self.phase.poll()) {
                Async::NotReady => return Ok(Async::NotReady),
                Async::Ready(Phase::A(listener)) => Phase::B(listener.incoming().into_future()),
                Async::Ready(Phase::B((client, incoming))) => {
                    let (connected, _) = track_try!(client.ok_or(ErrorKind::Invalid));

                    // TODO: support keep-alive
                    let router = self.router.clone();
                    let future = connected
                        .then(|r| Ok(track_try!(r)))
                        .map_err(|e: Error| e)
                        .and_then(|stream| {
                                      let connection =
                                          miasht::server::Connection::new(stream, 1024, 10240, 16);
                                      connection.read_request().then(|r| Ok(track_try!(r)))
                                  })
                        .and_then(move |request| router.handle_request(request))
                        .map_err(|e| panic!("{:?}", e)); // TODO: error handling
                    self.spawner.spawn(future);
                    Phase::B(incoming.into_future())
                }
                _ => unreachable!(),
            };
            self.phase = next;
        }
    }
}

fn request_to_rpc_request<P: Procedure>(url: &Url,
                                        request: &Request<TcpStream>,
                                        body: Vec<u8>)
                                        -> Result<P::Request> {
    let mut de = RequestDeserializer::new(P::entry_point(), url, request, body);
    track!(Deserialize::deserialize(&mut de))
}
fn rpc_response_to_response<P: Procedure>(connection: Connection<TcpStream>,
                                          rpc_response: P::Response)
                                    -> Result<(Response<TcpStream>, Vec<u8>)> {
    use serde::Serialize;
    let mut serializer = ResponseSerializer::new(connection);
    track_try!(rpc_response.serialize(&mut serializer));
    track!(serializer.finish())
}

type HandleRequestResult = BoxFuture<(Response<TcpStream>, Vec<u8>), Error>;
type HandleRequestFn =
    Box<Fn(Url, Request<TcpStream>, Vec<u8>) -> HandleRequestResult + Send + 'static>;

#[derive(Clone)]
struct RoutingTree {
    trie: Arc<Trie>,
}
unsafe impl Send for RoutingTree {}
impl RoutingTree {
    fn handle_request(self, request: Request<TcpStream>) -> BoxFuture<(), Error> {
        futures::done(request.into_body_reader())
            .and_then(|req| track_err!(req.read_all_bytes()))
            .and_then(move |(req, body)| {
                let req = req.into_inner();
                let base = Url::parse("http://localhost/").unwrap(); // TODO
                let url = Url::options()
                    .base_url(Some(&base))
                    .parse(req.path())
                    .expect("TODO: error handling");
                let mut trie = self.trie.root();
                for segment in url.path_segments().expect("TODO") {
                    if let Some(child) = trie.get_child(segment) {
                        trie = child;
                    } else {
                        panic!("TODO");
                    }
                }
                if let Some(handler) = trie.get_value(req.method()) {
                    handler(url, req, body).map_err(|_| unreachable!())
                } else {
                    panic!("TODO error handling: {:?}", url)
                }
            })
            .then(|result| match result {
                      Ok((response, body)) => {
                // TODO: suport keep-alive
                response
                    .write_all_bytes(body)
                    .map_err(|e| panic!("{:?}", e))
                    .and_then(|res| res)
                    .map_err(|e| panic!("{:?}", e))
                    .map(|_| ())
            }
                      Err(e) => panic!("TODO: {:?}", e),
                  })
            .boxed()
    }
}

 struct RoutingTreeBuilder {
    trie: Trie,
}
impl RoutingTreeBuilder {
    pub fn new() -> Self {
        RoutingTreeBuilder { trie: Trie::new() }
    }
    pub fn finish(self) -> RoutingTree {
        RoutingTree { trie: Arc::new(self.trie) }
    }
    pub fn register<H: Send + 'static>(&mut self, method: Method, entry_point: EntryPoint, handler: H) -> Result<()>
        where H: Fn(Url, Request<TcpStream>, Vec<u8>) -> HandleRequestResult
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
    pub fn insert(&mut self, method: Method, entry_point: &EntryPoint, handler: HandleRequestFn) -> Result<()> {
        let mut node = &mut self.root;
        for segment in entry_point.segments() {
            use types::PathSegment::*;
            let key = match *segment {
                Val(s) => Some(s),
                Var => None,
            };
            node = {
                    node
                }
                .children
                .entry(key)
                .or_insert_with(|| TrieNode::new());
        }
        track_assert!(!node.leafs.contains_key(&method),
                      ErrorKind::Invalid);
        node.leafs.insert(method, handler);
        Ok(())
    }
    pub fn root(&self) -> &TrieNode {
        &self.root
    }
}

struct TrieNode {
    children: HashMap<Option<&'static str>, TrieNode>,
    leafs: HashMap<Method, HandleRequestFn>,
}
impl TrieNode {
    pub fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            leafs: HashMap::new(),
        }
    }
    pub fn get_child<'a>(&'a self, segment: &str) -> Option<&'a Self> {
        let segment: &'static str = unsafe { &*(segment as *const _) as _ }; // TODO
        self.children
            .get(&Some(segment))
            .or_else(|| self.children.get(&None))
    }
    pub fn get_value(&self, method: Method) -> Option<&HandleRequestFn> {
        self.leafs.get(&method)
    }
}
