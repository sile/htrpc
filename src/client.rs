use std::net::SocketAddr;
use fibers::net::TcpStream;
use futures::{self, Future, Poll, Async, BoxFuture};
use handy_async::future::Phase;
use miasht;
use miasht::builtin::io::IoExt;
use miasht::builtin::futures::FutureExt;
use miasht::client::{Connection, Response};
use serde::ser::Serialize;

use Error;
use deserializers::RpcResponseDeserializer;
use procedure::Procedure;
use serializers::RequestSerializer;

// TODO: Support keep-alive
#[derive(Debug)]
pub struct RpcClient {
    server: SocketAddr,
}
impl RpcClient {
    pub fn new(server: SocketAddr) -> Self {
        RpcClient { server }
    }

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
                    let entry_point = P::entry_point();
                    let request = self.request.take().unwrap();
                    let mut ser = RequestSerializer::new(connection, P::method(), entry_point);
                    track_try!(request.serialize(&mut ser));
                    let (request, body) = track_try!(ser.finish());
                    Phase::B(request.write_all_bytes(body).and_then(|r| r).boxed())
                }
                Async::Ready(Phase::B(connection)) => Phase::C(connection.read_response().boxed()),
                Async::Ready(Phase::C(response)) => {
                    let future = futures::done(response.into_body_reader())
                        .and_then(|res| track_err!(res.read_all_bytes()))
                        .map(|(res, body)| (res.into_inner(), body));
                    Phase::D(future.boxed())
                }
                Async::Ready(Phase::D((response, body))) => {
                    use serde::Deserialize;
                    let mut deserializer = RpcResponseDeserializer::new(&response, body);
                    let response = track_try!(P::Response::deserialize(&mut deserializer));
                    return Ok(Async::Ready(response));
                }
                _ => unreachable!(),
            };
            self.phase = next;
        }
    }
}
