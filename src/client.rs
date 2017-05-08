use std::net::SocketAddr;
use futures::BoxFuture;

use Error;
use procedure::Procedure;

#[derive(Debug)]
pub struct RpcClient {
    server: SocketAddr,
}
impl RpcClient {
    pub fn new(server: SocketAddr) -> Self {
        RpcClient { server }
    }

    pub fn call<P>(&mut self, input: P::Input) -> BoxFuture<P::Output, Error>
        where P: Procedure
    {
        let _entry_point = P::entry_point();
        let _ = input;
        panic!()
    }
}
