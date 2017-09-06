//! Connection pool.
use std::cmp::Ordering;
use std::collections::{Bound, BTreeMap, HashMap};
use std::fmt;
use std::net::SocketAddr;
use std::time::{Duration, SystemTime};
use fibers::net::TcpStream;
use fibers::sync::mpsc;
use fibers::sync::oneshot;
use fibers::time::timer::{TimerExt, TimeoutAfter};
use futures::{self, Future, Async, Poll, Stream};
use futures::future::Done;
use handy_async::future::Phase;
use miasht;
use trackable::error::ErrorKindExt;

use {Error, ErrorKind, Procedure};
use client::CallInner;

type TcpConnection = miasht::client::Connection<TcpStream>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ConnectionId {
    addr: SocketAddr,
    seq_no: u64,
}
impl ConnectionId {
    pub fn new(addr: SocketAddr, seq_no: u64) -> Self {
        ConnectionId { addr, seq_no }
    }
}
impl PartialOrd for ConnectionId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ConnectionId {
    fn cmp(&self, other: &Self) -> Ordering {
        let other = (other.addr.ip(), other.addr.port(), other.seq_no);
        (self.addr.ip(), self.addr.port(), self.seq_no).cmp(&other)
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
    AddToBlacklist { addr: SocketAddr },
}

struct PooledConnection {
    on_error: Option<(SocketAddr, mpsc::Sender<Command>)>,
    phase: Phase<Done<TcpConnection, Error>, TimeoutAfter<miasht::client::Connect>>,
}
impl PooledConnection {
    fn failed(error: Error) -> Self {
        let phase = Phase::A(futures::failed(error));
        PooledConnection {
            on_error: None,
            phase,
        }
    }
}
impl Future for PooledConnection {
    type Item = TcpConnection;
    type Error = Error;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match track!(self.phase.poll().map_err(Error::from)) {
            Err(e) => {
                if let Some((addr, tx)) = self.on_error.take() {
                    let _ = tx.send(Command::AddToBlacklist { addr });
                }
                Err(e)
            }
            Ok(Async::Ready(Phase::A(connection))) => Ok(Async::Ready(connection)),
            Ok(Async::Ready(Phase::B(connection))) => Ok(Async::Ready(connection)),
            Ok(Async::Ready(_)) => unreachable!(),
            Ok(Async::NotReady) => Ok(Async::NotReady),
        }
    }
}
impl fmt::Debug for PooledConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PooledConnection {{ .. }}")
    }
}

/// This managements a pool of RPC clients.
#[derive(Debug)]
pub struct RpcClientPool {
    pool_size: usize,
    connections: BTreeMap<ConnectionId, TcpConnection>,
    lru_queue: BTreeMap<u64, SocketAddr>,
    command_tx: mpsc::Sender<Command>,
    command_rx: mpsc::Receiver<Command>,
    seq_no: u64,
    blacklist: HashMap<SocketAddr, SystemTime>,
    suspended_duration: Duration,
    connect_timeout: Duration,
}
impl RpcClientPool {
    /// Makes a new `RpcClientPool` with the default pool size (1024).
    pub fn new() -> Self {
        Self::with_pool_size(1024)
    }

    /// Makes a new `RpcClientPool` with the specified pool size.
    pub fn with_pool_size(pool_size: usize) -> Self {
        let (command_tx, command_rx) = mpsc::channel();
        RpcClientPool {
            pool_size,
            connections: BTreeMap::new(),
            lru_queue: BTreeMap::new(),
            command_tx,
            command_rx,
            seq_no: 0,
            blacklist: HashMap::new(),
            suspended_duration: Duration::from_secs(60),
            connect_timeout: Duration::from_secs(1),
        }
    }

    /// Returns a handle of this pool.
    pub fn handle(&self) -> RpcClientPoolHandle {
        RpcClientPoolHandle { command_tx: self.command_tx.clone() }
    }

    /// Sets the suspended duration of an erroneous TCP address.
    pub fn set_suspended_duration(&mut self, duration: Duration) {
        self.suspended_duration = duration;
    }

    /// Sets the timeout of a TCP connecting phase.
    pub fn set_connect_timeout(&mut self, timeout: Duration) {
        self.connect_timeout = timeout;
    }

    fn handle_command(&mut self, command: Command) {
        match command {
            Command::AcquireConnection { addr, reply } => {
                if let Some(suspended_until) = self.blacklist.remove(&addr) {
                    if suspended_until > SystemTime::now() {
                        self.blacklist.insert(addr, suspended_until);
                        let future = PooledConnection::failed(
                            ErrorKind::Other
                                .cause(format!(
                                    "The address {:?} is unavailable until {:?}",
                                    addr,
                                    suspended_until
                                ))
                                .into(),
                        );
                        let _ = reply.send(future);
                        return;
                    }
                }
                let future = self.acquire_connection(addr);
                let _ = reply.send(future);
            }
            Command::ReleaseConnection { addr, connection } => {
                self.release_connection(addr, connection);
            }
            Command::AddToBlacklist { addr } => {
                let suspended_until = SystemTime::now() + self.suspended_duration;
                self.blacklist.insert(addr, suspended_until);
            }
        }
    }
    fn acquire_connection(&mut self, addr: SocketAddr) -> PooledConnection {
        let lower = ConnectionId::new(addr, 0);
        if let Some(id) = self.connections
            .range((Bound::Included(lower), Bound::Unbounded))
            .map(|(id, _)| id)
            .cloned()
            .nth(0)
            .and_then(|id| if id.addr == addr { Some(id) } else { None })
        {
            self.lru_queue.remove(&id.seq_no);
            let connection = self.connections.remove(&id).expect("Never fails");
            let phase = Phase::A(futures::finished(connection));
            PooledConnection {
                phase,
                on_error: None,
            }
        } else {
            let phase = Phase::B(miasht::client::Client::new().connect(addr).timeout_after(
                self.connect_timeout,
            ));
            PooledConnection {
                phase,
                on_error: Some((addr, self.command_tx.clone())),
            }
        }
    }
    fn release_connection(&mut self, addr: SocketAddr, connection: TcpConnection) {
        let id = ConnectionId::new(addr, self.seq_no);
        self.seq_no += 1;

        self.lru_queue.insert(id.seq_no, id.addr);
        self.connections.insert(id, connection);
        self.drop_exceeded_lru_connections();
    }
    fn drop_exceeded_lru_connections(&mut self) {
        while self.connections.len() > self.pool_size {
            let id = self.lru_queue
                .iter()
                .map(|(seq_no, addr)| ConnectionId::new(*addr, *seq_no))
                .nth(0)
                .expect("Never failes");
            self.lru_queue.remove(&id.seq_no);
            self.connections.remove(&id);
        }
    }
}
impl Future for RpcClientPool {
    type Item = ();
    type Error = ();
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        while let Async::Ready(command) = self.command_rx.poll().expect("Never fails") {
            self.handle_command(command.expect("Never fails"));
        }
        Ok(Async::NotReady)
    }
}

/// A handle for `RpcClientPool`.
#[derive(Debug, Clone)]
pub struct RpcClientPoolHandle {
    command_tx: mpsc::Sender<Command>,
}
impl RpcClientPoolHandle {
    /// Acquires a RPC client from the pool.
    ///
    /// If there is no avaialable pooled client, new client will be created.
    pub fn client(&self, addr: SocketAddr) -> PooledRpcClient {
        PooledRpcClient { addr, handle: self }
    }

    fn acquire_connection(&self, addr: SocketAddr) -> AcquireConnection {
        let (reply, reply_rx) = oneshot::channel();
        let command = Command::AcquireConnection { addr, reply };
        let _ = self.command_tx.send(command);
        let phase = Phase::A(reply_rx);
        AcquireConnection { phase }
    }
    fn release_connection(&self, addr: SocketAddr, connection: TcpConnection) {
        let command = Command::ReleaseConnection { addr, connection };
        let _ = self.command_tx.send(command);
    }
}

/// Pooled RPC client.
#[derive(Debug)]
pub struct PooledRpcClient<'a> {
    addr: SocketAddr,
    handle: &'a RpcClientPoolHandle,
}
impl<'a> PooledRpcClient<'a> {
    /// Issues an RPC request and returns the `Future`
    /// which will result in the corresponding response.
    pub fn call<P>(&self, request: P::Request) -> Call<P>
    where
        P: Procedure,
    {
        let future = self.handle.acquire_connection(self.addr);
        let inner = CallInner {
            request: Some(request),
            phase: Phase::A(Box::new(future)),
        };
        let future = Call {
            inner,
            addr: self.addr,
            handle: self.handle.clone(),
        };
        future
    }
}

#[derive(Debug)]
struct AcquireConnection {
    phase: Phase<oneshot::Receiver<PooledConnection>, PooledConnection>,
}
impl Future for AcquireConnection {
    type Item = TcpConnection;
    type Error = Error;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        while let Async::Ready(phase) = track!(self.phase.poll().map_err(Error::from))? {
            let next = match phase {
                Phase::A(future) => Phase::B(future),
                Phase::B(connection) => return Ok(Async::Ready(connection)),
                _ => unreachable!(),
            };
            self.phase = next;
        }
        Ok(Async::NotReady)
    }
}

/// A `Future` which represents an RPC invocation.
pub struct Call<P: Procedure> {
    inner: CallInner<P>,
    addr: SocketAddr,
    handle: RpcClientPoolHandle,
}
impl<P> Future for Call<P>
where
    P: Procedure,
{
    type Item = P::Response;
    type Error = Error;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        if let Async::Ready((response, connection)) = track!(self.inner.poll())? {
            self.handle.release_connection(self.addr, connection);
            Ok(Async::Ready(response))
        } else {
            Ok(Async::NotReady)
        }
    }
}
