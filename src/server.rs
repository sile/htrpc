use std::net::SocketAddr;
use fibers::{self, Spawn, BoxSpawn};
use fibers::net::TcpStream;
use fibers::net::futures::Connected;
use futures::{Future, Poll, Async, Stream, BoxFuture};
use futures::stream::StreamFuture;
use handy_async::future::Phase;
use miasht;
use miasht::builtin::io::IoExt;
use miasht::builtin::futures::FutureExt;
use miasht::server::{Connection, Request, Response};
use serde::{Deserialize, Serialize};
use url::Url;

use {Result, Error, ErrorKind};
use deserializers::RpcRequestDeserializer;
use procedure::{Procedure, HandleRpc};
use router::{Router, RouterBuilder, RouteError};
use serializers::RpcResponseSerializer;

/// The `RpcServer` builder.
pub struct RpcServerBuilder {
    bind_addr: SocketAddr,
    router: RouterBuilder,
}
impl RpcServerBuilder {
    /// Makes a new `RpcServerBuilder` instance.
    pub fn new(bind_addr: SocketAddr) -> Self {
        RpcServerBuilder {
            bind_addr,
            router: RouterBuilder::new(),
        }
    }

    /// Registers an RPC handler.
    pub fn register<P, H>(&mut self, handler: H) -> Result<()>
        where P: Procedure,
              H: HandleRpc<P>
    {
        let handle_http_request = move |url, http_request, body| {
            let handler = handler.clone();
            let rpc_request = {
                let mut de =
                    RpcRequestDeserializer::new(P::entry_point(), &url, &http_request, body);
                match track!(Deserialize::deserialize(&mut de)) {
                    Err(e) => {
                        //
                        panic!("TODO: response 400");
                    }
                    Ok(r) => r,
                }
            };
            handler
                .handle_rpc(rpc_request)
                .map_err(|_| unreachable!())
                .and_then(move |rpc_response| {
                              let (http_response, body) = {
                                  let mut ser = RpcResponseSerializer::new(http_request.finish());
                                  track_try!(rpc_response.serialize(&mut ser));
                                  track_try!(ser.finish())
                              };
                              Ok((http_response, body))
                          })
                .boxed()
        };
        track_try!(self.router
                       .register_handler(P::method(), P::entry_point(), handle_http_request));
        Ok(())
    }

    /// Starts the `Future` which represents the RPC server.
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

/// RPC Server.
pub struct RpcServer {
    spawner: BoxSpawn,
    router: Router,
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

                    let future = HandleHttpRequest {
                        router: self.router.clone(),
                        phase: Phase::A(connected),
                    };
                    self.spawner.spawn(future);
                    Phase::B(incoming.into_future())
                }
                _ => unreachable!(),
            };
            self.phase = next;
        }
    }
}

struct HandleHttpRequest {
    router: Router,
    phase: Phase<Connected,
                 BoxFuture<(Request<TcpStream>, Vec<u8>), miasht::Error>,
                 BoxFuture<(Response<TcpStream>, Vec<u8>), Error>,
                 BoxFuture<(), miasht::Error>>,
}
impl HandleHttpRequest {
    fn poll_impl(&mut self) -> Poll<(), Error> {
        loop {
            let next = match track_try!(self.phase.poll()) {
                Async::NotReady => return Ok(Async::NotReady),
                Async::Ready(Phase::A(stream)) => {
                    let connection = Connection::new(stream, 1024, 8096, 32);
                    let future = connection
                        .read_request()
                        .and_then(|req| req.into_body_reader())
                        .and_then(|req| req.read_all_bytes())
                        .map(|(req, body)| (req.into_inner(), body));
                    Phase::B(future.boxed())
                }
                Async::Ready(Phase::B((request, body))) => {
                    let base = Url::parse("http://localhost/").expect("Never fails");
                    // TODO: Handle InvalidRequest
                    let url =
                        track_try!(Url::options().base_url(Some(&base)).parse(request.path()));
                    let future = match self.router.route(&url, &request) {
                        Err(RouteError::NotFound) => panic!("TODO"),
                        Err(RouteError::MethodNotAllowed) => panic!("TODO"),
                        Ok(handler) => handler(url, request, body).map_err(|_| unreachable!()),
                    };
                    Phase::C(future.boxed())
                }
                Async::Ready(Phase::C((response, body))) => {
                    let future = response
                        .write_all_bytes(body)
                        .and_then(|res| res)
                        .map(|_| ());
                    Phase::D(future.boxed())
                }
                Async::Ready(Phase::D(())) => {
                    return Ok(Async::Ready(()));
                }
                _ => unreachable!(),
            };
            self.phase = next;
        }
    }
}
impl Future for HandleHttpRequest {
    type Item = ();
    type Error = ();
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.poll_impl()
            .map_err(|_| {
                         // TODO: logging
                         ()
                     })
    }
}
