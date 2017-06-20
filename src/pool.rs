#![allow(missing_docs)]
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::time::Instant;
use fibers::net::TcpStream;
use fibers::sync::mpsc;
use fibers::sync::oneshot;
use miasht;

use Procedure;

type TcpConnection = miasht::client::Connection<TcpStream>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ConnectionId {
    addr: SocketAddr,
    created_at: Instant,
}
impl ConnectionId {
    pub fn new(addr: SocketAddr) -> Self {
        ConnectionId {
            addr,
            created_at: Instant::now(),
        }
    }
}
impl PartialOrd for ConnectionId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ConnectionId {
    fn cmp(&self, other: &Self) -> Ordering {
        let other = (other.addr.ip(), other.addr.port(), other.created_at);
        (self.addr.ip(), self.addr.port(), self.created_at).cmp(&other)
    }
}

#[derive(Debug)]
enum Command {
    AcquireConnection {
        addr: SocketAddr,
        reply: oneshot::Sender<PooledConnection>,
    },
    ReleaseConnection {
        addr: SocketAddr,
        connection: TcpConnection,
    },
}

#[derive(Debug)]
pub struct PooledConnection;

#[derive(Debug)]
pub struct RpcClientPool {
    pool_size: usize,
    connections: BTreeMap<ConnectionId, TcpConnection>,
    lru_queue: BTreeMap<Instant, SocketAddr>,
    command_tx: mpsc::Sender<Command>,
    command_rx: mpsc::Receiver<Command>,
}
impl RpcClientPool {
    pub fn new() -> Self {
        Self::with_pool_size(1024)
    }
    pub fn with_pool_size(pool_size: usize) -> Self {
        let (command_tx, command_rx) = mpsc::channel();
        RpcClientPool {
            pool_size,
            connections: BTreeMap::new(),
            lru_queue: BTreeMap::new(),
            command_tx,
            command_rx,
        }
    }
    pub fn handle(&self) -> RpcClientPoolHandle {
        RpcClientPoolHandle { command_tx: self.command_tx.clone() }
    }
}

#[derive(Debug, Clone)]
pub struct RpcClientPoolHandle {
    command_tx: mpsc::Sender<Command>,
}
impl RpcClientPoolHandle {
    pub fn client(&self, addr: SocketAddr) -> PooledRpcClient {
        PooledRpcClient {
            addr,
            command_tx: &self.command_tx,
        }
    }
}

#[derive(Debug)]
pub struct PooledRpcClient<'a> {
    addr: SocketAddr,
    command_tx: &'a mpsc::Sender<Command>,
}
impl<'a> PooledRpcClient<'a> {
    /// Issues an RPC request and returns the `Future`
    /// which will result in the corresponding response.
    pub fn call<P>(&self, request: P::Request) -> Call<P>
    where
        P: Procedure,
    {
        panic!()
    }
}

#[derive(Debug)]
pub struct Call<P>(P);
