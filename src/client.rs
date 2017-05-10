use std::net::SocketAddr;
use fibers::net::TcpStream;
use futures::{Future, Poll, Async, BoxFuture};
use handy_async::future::Phase;
use miasht;
use miasht::builtin::headers::{ContentLength, ContentType};
// use miasht::builtin::io::IoExt;
use miasht::builtin::futures::FutureExt;
use serde::ser::Serialize;
use url::{self, Url};

use Error;
use procedure::{Procedure, RpcInput};
use serializers::{UrlPathSerializer, UrlQuerySerializer, HttpHeaderSerializer};

// TODO: Support keep-alive
#[derive(Debug)]
pub struct RpcClient {
    server: SocketAddr,
}
impl RpcClient {
    pub fn new(server: SocketAddr) -> Self {
        RpcClient { server }
    }

    pub fn call<P>(&mut self, input: P::Input) -> Call<P>
        where P: Procedure
    {
        let client = miasht::Client::new();
        let future = Call {
            input: Some(input),
            phase: Phase::A(client.connect(self.server)),
        };
        future
    }
}

pub struct Call<P>
    where P: Procedure
{
    input: Option<P::Input>,
    phase: Phase<miasht::client::Connect,
                 BoxFuture<miasht::client::Connection<TcpStream>, miasht::Error>,
                 BoxFuture<miasht::client::Response<TcpStream>, miasht::Error>>,
}
impl<P> Future for Call<P>
    where P: Procedure
{
    type Item = P::Output;
    type Error = Error;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            let next = match track_try!(self.phase.poll()) {
                Async::NotReady => return Ok(Async::NotReady),
                Async::Ready(Phase::A(connection)) => {
                    let entry_point = P::entry_point();
                    let input = self.input.take().unwrap();
                    let (path_args, query_args, header_args, body) = track_try!(input.decompose());

                    // TODO: Operate directory on url-path
                    let mut dummy_url = Url::parse("http://localhost/").unwrap();
                    {
                        let mut serializer =
                            UrlPathSerializer::new(&entry_point.path,
                                                   dummy_url.path_segments_mut().unwrap());
                        track_try!(path_args.serialize(&mut serializer));
                    }
                    {
                        let mut serializer = UrlQuerySerializer::new(dummy_url.query_pairs_mut());
                        track_try!(query_args.serialize(&mut serializer));
                    }
                    let relative_url = &dummy_url[url::Position::BeforePath..];
                    let mut builder = connection.build_request(entry_point.method, relative_url);
                    {
                        let mut serializer = HttpHeaderSerializer::new(builder.headers_mut());
                        track_try!(header_args.serialize(&mut serializer));
                    }

                    let body_bytes = track_try!(P::Input::serialize_body(body));
                    if let Some(mime) = P::Input::content_type() {
                        builder.add_header(&ContentType(mime));
                    }
                    builder.add_header(&ContentLength(body_bytes.len() as u64));

                    let request = builder.finish();
                    Phase::B(request
                                 .write_all_bytes(body_bytes)
                                 .and_then(|r| r)
                                 .boxed())
                }
                Async::Ready(Phase::B(connection)) => Phase::C(connection.read_response().boxed()),
                Async::Ready(Phase::C(_response)) => {
                    // TODO
                    panic!()
                }
                _ => unreachable!(),
            };
            self.phase = next;
        }
    }
}
