use std::net::SocketAddr;
use fibers::net::TcpStream;
use futures::{self, Future, Poll, Async, BoxFuture};
use handy_async::future::Phase;
use miasht;
use miasht::builtin::io::IoExt;
use miasht::builtin::futures::FutureExt;
use miasht::client::{Connection, Response};
use serde::{Deserialize, Serialize};

use Error;
use deserializers::RpcResponseDeserializer;
use procedure::Procedure;
use serializers::RpcRequestSerializer;
use types::HttpMethod;

/// RPC Client.
#[derive(Debug)]
pub struct RpcClient {
    server: SocketAddr,
}
impl RpcClient {
    /// Makes an RPC client which will communicate with the `server`.
    pub fn new(server: SocketAddr) -> Self {
        RpcClient { server }
    }

    /// Issues an RPC request and returns the `Future`
    /// which will result in the corresponding response.
    pub fn call<P>(&mut self, request: P::Request) -> Call<P>
    where
        P: Procedure,
    {
        let client = miasht::Client::new();
        let future = Call(CallInner {
            request: Some(request),
            phase: Phase::A(client.connect(self.server).map_err(Error::from).boxed()),
        });
        future
    }
}

/// A `Future` which represents an RPC invocation.
pub struct Call<P>(CallInner<P>)
where
    P: Procedure;
impl<P> Future for Call<P>
where
    P: Procedure,
{
    type Item = P::Response;
    type Error = Error;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        if let Async::Ready((response, _)) = track!(self.0.poll())? {
            Ok(Async::Ready(response))
        } else {
            Ok(Async::NotReady)
        }
    }
}

pub(crate) struct CallInner<P>
where
    P: Procedure,
{
    pub request: Option<P::Request>,
    pub phase: Phase<
        BoxFuture<Connection<TcpStream>, Error>,
        BoxFuture<Connection<TcpStream>, miasht::Error>,
        BoxFuture<Response<TcpStream>, miasht::Error>,
        BoxFuture<(Response<TcpStream>, Vec<u8>), miasht::Error>,
    >,
}
impl<P> Future for CallInner<P>
where
    P: Procedure,
{
    type Item = (P::Response, Connection<TcpStream>);
    type Error = Error;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            let next = match track!(self.phase.poll().map_err(Error::from))? {
                Async::NotReady => return Ok(Async::NotReady),
                Async::Ready(Phase::A(connection)) => {
                    // Writes HTTP request.
                    let entry_point = P::entry_point();
                    let rpc_request = self.request.take().expect("Never fail");
                    let mut ser = RpcRequestSerializer::new(connection, P::method(), entry_point);
                    track!(rpc_request.serialize(&mut ser))?;
                    let (request, body) = track!(ser.finish())?;
                    Phase::B(request.write_all_bytes(body).and_then(|r| r).boxed())
                }
                Async::Ready(Phase::B(connection)) => {
                    // Reads HTTP response (without body).
                    Phase::C(connection.read_response().boxed())
                }
                Async::Ready(Phase::C(response)) => {
                    // Reads HTTP response body.
                    let future = if P::method() == HttpMethod::Head {
                        futures::finished((response, Vec::new())).boxed()
                    } else {
                        futures::done(response.into_body_reader())
                            .and_then(|res| res.read_all_bytes().map_err(|e| track!(e)))
                            .map(|(res, body)| (res.into_inner(), body))
                            .boxed()
                    };
                    Phase::D(future)
                }
                Async::Ready(Phase::D((response, body))) => {
                    // Converts from HTTP response to RPC response.
                    let rpc_response = {
                        let mut deserializer = RpcResponseDeserializer::new(&response, body);
                        track!(P::Response::deserialize(&mut deserializer))?
                    };
                    return Ok(Async::Ready((rpc_response, response.finish())));
                }
                _ => unreachable!(),
            };
            self.phase = next;
        }
    }
}
