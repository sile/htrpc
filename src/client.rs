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
        where P: Procedure
    {
        let client = miasht::Client::new();
        let future = Call {
            request: Some(request),
            phase: Phase::A(client.connect(self.server)),
        };
        future
    }
}

/// A `Future` which represents an RPC invocation.
pub struct Call<P>
    where P: Procedure
{
    request: Option<P::Request>,
    phase: Phase<miasht::client::Connect,
                 BoxFuture<Connection<TcpStream>, miasht::Error>,
                 BoxFuture<Response<TcpStream>, miasht::Error>,
                 BoxFuture<(Response<TcpStream>, Vec<u8>), miasht::Error>>,
}
impl<P> Future for Call<P>
    where P: Procedure
{
    type Item = P::Response;
    type Error = Error;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            let next = match track_try!(self.phase.poll()) {
                Async::NotReady => return Ok(Async::NotReady),
                Async::Ready(Phase::A(connection)) => {
                    // Writes HTTP request.
                    let entry_point = P::entry_point();
                    let rpc_request = self.request.take().expect("Never fail");
                    let mut ser = RpcRequestSerializer::new(connection, P::method(), entry_point);
                    track_try!(rpc_request.serialize(&mut ser));
                    let (request, body) = track_try!(ser.finish());
                    Phase::B(request.write_all_bytes(body).and_then(|r| r).boxed())
                }
                Async::Ready(Phase::B(connection)) => {
                    // Reads HTTP response (without body).
                    Phase::C(connection.read_response().boxed())
                }
                Async::Ready(Phase::C(response)) => {
                    // Reads HTTP response body.
                    let future = futures::done(response.into_body_reader())
                        .and_then(|res| track_err!(res.read_all_bytes()))
                        .map(|(res, body)| (res.into_inner(), body));
                    Phase::D(future.boxed())
                }
                Async::Ready(Phase::D((response, body))) => {
                    // Converts from HTTP response to RPC response.
                    let mut deserializer = RpcResponseDeserializer::new(&response, body);
                    let rpc_response = track_try!(P::Response::deserialize(&mut deserializer));
                    return Ok(Async::Ready(rpc_response));
                }
                _ => unreachable!(),
            };
            self.phase = next;
        }
    }
}
