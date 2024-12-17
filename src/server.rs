use fibers::net::TcpStream;
use fibers::{self, BoxSpawn, Spawn};
use futures::stream::StreamFuture;
use futures::{self, Async, Future, Poll, Stream};
use handy_async::future::Phase;
use miasht;
use miasht::builtin::futures::FutureExt;
use miasht::builtin::io::IoExt;
use miasht::server::{Connection, Request, Response};
use serde::Deserialize;
use slog::{Discard, Logger};
use std::io;
use std::net::SocketAddr;
use trackable::error::ErrorKindExt;

use deserializers::RpcRequestDeserializer;
use misc;
use procedure::{HandleRpc, Procedure};
use rfc7807::Problem;
use router::{Router, RouterBuilder};
use serializers::RpcResponseSerializer;
use types::{HttpMethod, HttpStatus};
use {Error, ErrorKind, Result};

type BoxFuture<T, E> = Box<dyn Future<Item = T, Error = E> + Send + 'static>;

/// The `RpcServer` builder.
pub struct RpcServerBuilder {
    bind_addr: SocketAddr,
    logger: Logger,
    router: RouterBuilder,
}
impl RpcServerBuilder {
    /// Makes a new `RpcServerBuilder` instance.
    pub fn new(bind_addr: SocketAddr) -> Self {
        RpcServerBuilder {
            bind_addr,
            logger: Logger::root(Discard, o!()),
            router: RouterBuilder::new(),
        }
    }

    /// Sets the logger to this server.
    pub fn set_logger(&mut self, logger: Logger) {
        self.logger = logger;
    }

    /// Registers an RPC handler.
    pub fn register<P, H>(&mut self, handler: H, _: P) -> Result<()>
    where
        P: Procedure,
        H: HandleRpc<P>,
    {
        use RpcRequest;
        let handle_http_request = move |url, http_request| {
            let handler = handler.clone();
            let rpc_request: P::Request = {
                let deserialize_result = {
                    let mut de = RpcRequestDeserializer::new(P::entry_point(), &url, &http_request);
                    track!(Deserialize::deserialize(&mut de))
                };
                match deserialize_result {
                    Err(e) => {
                        let future = futures::done(http_request.into_body_reader())
                            .map_err(Error::from)
                            .and_then(|request| request.read_all_bytes().map_err(Error::from))
                            .and_then(move |(request, _)| {
                                let http_request = request.into_inner();
                                let rpc_response =
                                    Problem::trackable(HttpStatus::BadRequest, e).into_response();
                                track!(RpcResponseSerializer::serialize(
                                    rpc_response,
                                    http_request.finish(),
                                ))
                            });
                        let future: BoxFuture<_, _> = Box::new(future);
                        return future;
                    }
                    Ok(r) => r,
                }
            };
            let future = futures::done(http_request.into_body_reader())
                .map_err(Error::from)
                .and_then(move |http_request| rpc_request.read_body(http_request))
                .and_then(move |(http_request, rpc_request)| {
                    let http_request = http_request.into_inner();
                    handler
                        .handle_rpc(rpc_request)
                        .map_err(|_| unreachable!())
                        .and_then(move |rpc_response| {
                            track!(RpcResponseSerializer::serialize(
                                rpc_response,
                                http_request.finish(),
                            ))
                        })
                });
            let future: BoxFuture<_, _> = Box::new(future);
            future
        };
        track!(self
            .router
            .register_handler(P::method(), P::entry_point(), handle_http_request,))?;
        Ok(())
    }

    /// Starts the `Future` which represents the RPC server.
    pub fn start<S>(self, spawner: S) -> RpcServer
    where
        S: Spawn + Send + 'static,
    {
        RpcServer {
            spawner: spawner.boxed(),
            logger: self.logger,
            router: self.router.finish(),
            phase: Phase::A(fibers::net::TcpListener::bind(self.bind_addr)),
        }
    }
}

/// RPC Server.
pub struct RpcServer {
    spawner: BoxSpawn,
    logger: Logger,
    router: Router,
    phase:
        Phase<fibers::net::futures::TcpListenerBind, StreamFuture<fibers::net::streams::Incoming>>,
}
impl Future for RpcServer {
    type Item = ();
    type Error = Error;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            let next = match track!(self.phase.poll().map_err(Error::from))? {
                Async::NotReady => return Ok(Async::NotReady),
                Async::Ready(Phase::A(listener)) => {
                    info!(
                        self.logger,
                        "RPC server started: {:?}",
                        listener.local_addr()
                    );
                    Phase::B(listener.incoming().into_future())
                }
                Async::Ready(Phase::B((client, incoming))) => {
                    let (connected, addr) =
                        track!(client.ok_or_else(|| ErrorKind::Invalid.error()))?;
                    debug!(self.logger, "New client is connected: {}", addr);

                    let connected = connected.then(|result| {
                        let stream = track!(result.map_err(miasht::Error::from))?;
                        let _ = stream.with_inner(|inner| inner.set_nodelay(true));
                        Ok(Connection::new(stream, 1024, 8096, 32))
                    });
                    let future = HandleHttpRequest {
                        logger: self.logger.clone(),
                        router: self.router.clone(),
                        phase: Phase::A(Box::new(connected)),
                        method: HttpMethod::Get, // Dummy
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
    logger: Logger,
    router: Router,
    phase: Phase<
        BoxFuture<Connection<TcpStream>, miasht::Error>,
        BoxFuture<Option<Request<TcpStream>>, miasht::Error>,
        BoxFuture<(Response<TcpStream>, Box<dyn AsRef<[u8]> + Send + 'static>), Error>,
    >,
    method: HttpMethod,
}
impl HandleHttpRequest {
    fn poll_impl(&mut self) -> Poll<(), Error> {
        loop {
            let next = match track!(self.phase.poll().map_err(Error::from))? {
                Async::NotReady => return Ok(Async::NotReady),
                Async::Ready(Phase::A(connection)) => {
                    let future = connection.read_request().map(Some).or_else(|e| {
                        if let Some(e) = e.concrete_cause::<io::Error>() {
                            if e.kind() == io::ErrorKind::UnexpectedEof
                                || e.kind() == io::ErrorKind::ConnectionReset
                            {
                                // The connection is reset by the peer.
                                return Ok(None);
                            }
                        }
                        Err(e)
                    });
                    let future: BoxFuture<_, _> = Box::new(future);
                    Phase::B(future)
                }
                Async::Ready(Phase::B(None)) => {
                    return Ok(Async::Ready(()));
                }
                Async::Ready(Phase::B(Some(request))) => {
                    debug!(
                        self.logger,
                        "RPC request: method={}, path={:?}",
                        request.method(),
                        request.path(),
                    );
                    self.method = request.method();
                    let future: BoxFuture<_, _> =
                        match track!(misc::parse_relative_url(request.path())) {
                            Err(e) => {
                                let future = futures::done(request.into_body_reader())
                                    .map_err(Error::from)
                                    .and_then(|request| {
                                        request.read_all_bytes().map_err(Error::from)
                                    })
                                    .and_then(|(request, _)| {
                                        let request = request.into_inner();
                                        let rpc_response =
                                            Problem::trackable(HttpStatus::BadRequest, e)
                                                .into_response();
                                        track!(RpcResponseSerializer::serialize(
                                            rpc_response,
                                            request.finish(),
                                        ))
                                    });
                                Box::new(future)
                            }
                            Ok(url) => match self.router.route(&url, &request) {
                                Err(status) => {
                                    let future = futures::done(request.into_body_reader())
                                        .map_err(Error::from)
                                        .and_then(|request| {
                                            request.read_all_bytes().map_err(Error::from)
                                        })
                                        .and_then(move |(request, _)| {
                                            let request = request.into_inner();
                                            let rpc_response =
                                                Problem::about_blank(status).into_response();
                                            track!(RpcResponseSerializer::serialize(
                                                rpc_response,
                                                request.finish(),
                                            ))
                                        });
                                    Box::new(future)
                                }
                                Ok(handler) => handler(url, request),
                            },
                        };
                    Phase::C(future)
                }
                Async::Ready(Phase::C((response, body))) => {
                    let future: BoxFuture<_, _> = if self.method == HttpMethod::Head {
                        Box::new(response)
                    } else {
                        struct Temp(Box<dyn AsRef<[u8]> + Send + 'static>);
                        impl AsRef<[u8]> for Temp {
                            fn as_ref(&self) -> &[u8] {
                                (*self.0).as_ref()
                            }
                        }
                        Box::new(response.write_all_bytes(Temp(body)).and_then(|res| res))
                    };
                    Phase::A(future)
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
        self.poll_impl().map_err(|e| {
            warn!(self.logger, "Failed to handle RPC request: {}", e);
        })
    }
}
