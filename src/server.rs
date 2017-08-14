use std::io;
use std::net::SocketAddr;
use fibers::{self, Spawn, BoxSpawn};
use fibers::net::TcpStream;
use futures::{self, Future, Poll, Async, Stream, BoxFuture};
use futures::stream::StreamFuture;
use handy_async::future::Phase;
use miasht;
use miasht::builtin::io::IoExt;
use miasht::builtin::futures::FutureExt;
use miasht::server::{Connection, Request, Response};
use serde::Deserialize;
use slog::{Logger, Discard};
use trackable::error::ErrorKindExt;

use {Result, Error, ErrorKind};
use deserializers::RpcRequestDeserializer;
use misc;
use procedure::{Procedure, HandleRpc};
use rfc7807::Problem;
use router::{Router, RouterBuilder};
use serializers::RpcResponseSerializer;
use types::{HttpStatus, HttpMethod};

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
    pub fn register<P, H>(&mut self, handler: H) -> Result<()>
    where
        P: Procedure,
        H: HandleRpc<P>,
    {
        let handle_http_request = move |url, http_request, body| {
            let handler = handler.clone();
            let rpc_request = {
                let deserialize_result = {
                    let mut de =
                        RpcRequestDeserializer::new(P::entry_point(), &url, &http_request, body);
                    track!(Deserialize::deserialize(&mut de))
                };
                match deserialize_result {
                    Err(e) => {
                        let rpc_response = Problem::trackable(HttpStatus::BadRequest, e)
                            .into_response();
                        let result = track!(RpcResponseSerializer::serialize(
                            rpc_response,
                            http_request.finish(),
                        ));
                        return futures::done(result).boxed();
                    }
                    Ok(r) => r,
                }
            };
            handler
                .handle_rpc(rpc_request)
                .map_err(|_| unreachable!())
                .and_then(move |rpc_response| {
                    track!(RpcResponseSerializer::serialize(
                        rpc_response,
                        http_request.finish(),
                    ))
                })
                .boxed()
        };
        track!(self.router.register_handler(
            P::method(),
            P::entry_point(),
            handle_http_request,
        ))?;
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
                        Ok(Connection::new(stream, 1024, 8096, 32))
                    });
                    let future = HandleHttpRequest {
                        logger: self.logger.clone(),
                        router: self.router.clone(),
                        phase: Phase::A(connected.boxed()),
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
        BoxFuture<Option<(Request<TcpStream>, Vec<u8>)>, miasht::Error>,
        BoxFuture<(Response<TcpStream>, Vec<u8>), Error>,
    >,
    method: HttpMethod,
}
impl HandleHttpRequest {
    fn poll_impl(&mut self) -> Poll<(), Error> {
        loop {
            let next = match track!(self.phase.poll().map_err(Error::from))? {
                Async::NotReady => return Ok(Async::NotReady),
                Async::Ready(Phase::A(connection)) => {
                    let future = connection
                        .read_request()
                        .and_then(|req| req.into_body_reader())
                        .and_then(|req| req.read_all_bytes())
                        .map(|(req, body)| Some((req.into_inner(), body)))
                        .or_else(|e| {
                            if let Some(e) = e.concrete_cause::<io::Error>() {
                                if e.kind() == io::ErrorKind::UnexpectedEof ||
                                    e.kind() == io::ErrorKind::ConnectionReset
                                {
                                    // The connection is reset by the peer.
                                    return Ok(None);
                                }
                            }
                            Err(e)
                        });
                    Phase::B(future.boxed())
                }
                Async::Ready(Phase::B(None)) => {
                    return Ok(Async::Ready(()));
                }
                Async::Ready(Phase::B(Some((request, body)))) => {
                    debug!(
                        self.logger,
                        "RPC request: method={}, path={:?}, body_bytes={}",
                        request.method(),
                        request.path(),
                        body.len()
                    );
                    self.method = request.method();
                    let future = match track!(misc::parse_relative_url(request.path())) {
                        Err(e) => {
                            let rpc_response = Problem::trackable(HttpStatus::BadRequest, e)
                                .into_response();
                            let result = track!(RpcResponseSerializer::serialize(
                                rpc_response,
                                request.finish(),
                            ));
                            futures::done(result).boxed()
                        }
                        Ok(url) => {
                            match self.router.route(&url, &request) {
                                Err(status) => {
                                    let rpc_response = Problem::about_blank(status).into_response();
                                    let result = track!(RpcResponseSerializer::serialize(
                                        rpc_response,
                                        request.finish(),
                                    ));
                                    futures::done(result).boxed()
                                }
                                Ok(handler) => handler(url, request, body),
                            }
                        }
                    };
                    Phase::C(future)
                }
                Async::Ready(Phase::C((response, body))) => {
                    let future = if self.method == HttpMethod::Head {
                        response.boxed()
                    } else {
                        response.write_all_bytes(body).and_then(|res| res).boxed()
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
            ()
        })
    }
}
