use std::net::SocketAddr;
use fibers::net::TcpStream;
use futures::{self, Async, Future, Poll};
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

type BoxFuture<T, E> = Box<Future<Item = T, Error = E> + Send + 'static>;

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
            phase: Phase::A(Box::new(client.connect(self.server).map_err(Error::from))),
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
                    use RpcRequest;
                    let entry_point = P::entry_point();
                    let mut rpc_request = self.request.take().expect("Never fail");
                    let mut ser = RpcRequestSerializer::new(connection, P::method(), entry_point);
                    track!(rpc_request.serialize(&mut ser))?;
                    let body = rpc_request.body();
                    let request = track!(ser.finish(&body))?;
                    let future: BoxFuture<_, _> =
                        Box::new(request.write_all_bytes(body).and_then(|r| r));
                    Phase::B(future)
                }
                Async::Ready(Phase::B(connection)) => {
                    // Reads HTTP response (without body).
                    let future: BoxFuture<_, _> = Box::new(connection.read_response());
                    Phase::C(future)
                }
                Async::Ready(Phase::C(response)) => {
                    // Reads HTTP response body.
                    let future: BoxFuture<_, _> = if P::method() == HttpMethod::Head {
                        Box::new(futures::finished((response, Vec::new())))
                    } else {
                        let future = futures::done(response.into_body_reader())
                            .and_then(|res| res.read_all_bytes().map_err(|e| track!(e)))
                            .map(|(res, body)| (res.into_inner(), body));
                        Box::new(future)
                    };
                    Phase::D(future)
                }
                Async::Ready(Phase::D((response, body))) => {
                    // Converts from HTTP response to RPC response.
                    use RpcResponse;
                    let mut rpc_response = {
                        let mut deserializer = RpcResponseDeserializer::new(&response);
                        track!(P::Response::deserialize(&mut deserializer))?
                    };
                    rpc_response.set_body(body);
                    return Ok(Async::Ready((rpc_response, response.finish())));
                }
                _ => unreachable!(),
            };
            self.phase = next;
        }
    }
}
