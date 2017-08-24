//! HTTP based RPC library.
//!
//! This crate provides a thin framework to easily implement type-safe RPC channels
//! for client/server model communication.
#![warn(missing_docs)]
extern crate fibers;
extern crate futures;
extern crate handy_async;
extern crate miasht;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serdeconv;
#[macro_use]
extern crate slog;
#[macro_use]
extern crate trackable;
extern crate url;
extern crate url_serde;

pub use miasht::builtin::futures::FutureExt;

#[allow(missing_docs)]
pub type BodyReader =
    miasht::builtin::io::BodyReader<miasht::server::Request<fibers::net::TcpStream>>;

#[allow(missing_docs)]
pub fn content_length(body: &BodyReader) -> Option<u64> {
    if let miasht::builtin::io::BodyReader::FixedLength(ref r) = *body {
        Some(r.limit())
    } else {
        None
    }
}

#[allow(missing_docs)]
pub type ReadBody<T> = futures::BoxFuture<(BodyReader, T), Error>;

pub use client::RpcClient;
pub use error::{Error, ErrorKind};
pub use procedure::{Procedure, HandleRpc, RpcRequest, RpcResponse};
pub use server::{RpcServer, RpcServerBuilder};

/// A helper macro to construct an `EntryPoint` instance.
///
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate htrpc;
/// use htrpc::types::{EntryPoint, PathSegment};
///
/// # fn main() {
/// static SEGMENTS: &[PathSegment] =
///     &[PathSegment::Val("foo"), PathSegment::Var, PathSegment::Val("baz")];
/// let p0 = EntryPoint::new(SEGMENTS);
/// let p1 = htrpc_entry_point!["foo", _, "baz"];
/// assert_eq!(p0, p1);
/// # }
/// ```
#[macro_export]
macro_rules! htrpc_entry_point {
    ($($segment:tt),*) => {
        {
            static SEGMENTS: &[$crate::types::PathSegment] =
                &[$(htrpc_expand_segment!($segment)),*];
            $crate::types::EntryPoint::new(SEGMENTS)
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! htrpc_expand_segment {
    (_) => {
        $crate::types::PathSegment::Var
    };
    ($s:expr) => {
        $crate::types::PathSegment::Val($s)
    }
}

pub mod deserializers;
pub mod json;
pub mod json_pretty;
pub mod msgpack;
pub mod pool;
pub mod rfc7807;
pub mod serializers;
pub mod types;

mod client;
mod error;
mod misc;
mod procedure;
mod router;
mod server;

/// This crate specific `Result` type.
pub type Result<T> = ::std::result::Result<T, Error>;
