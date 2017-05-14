//! HTTP based RPC library.
//!
//! This crate provides a thin framework to easily implement type-safe RPC channels
//! for client/server model communication.
#![warn(missing_docs)]
extern crate fibers;
extern crate futures;
extern crate handy_async;
extern crate miasht;
#[macro_use]
extern crate serde;
extern crate serdeconv;
#[cfg(test)]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate trackable;
extern crate url;

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
pub mod serializers;
pub mod types;

mod client;
mod error;
mod procedure;
mod router;
mod server;

/// This crate specific `Result` type.
pub type Result<T> = ::std::result::Result<T, Error>;
